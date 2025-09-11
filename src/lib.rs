//! Defines a `#[nounwind]` attribute macro that prevents panics from unwinding,
//! similar to the C++ [`noexcept` specifier].
//!
//! This is clearer than using a drop guard,
//! and in some versions of Rust can provide a better error message.
//! In particular, aborting panics will often print a messages like "panic in a function that cannot unwind",
//! although this behavior is version-specific and not guaranteed to occur.
//!
//! The crate also provides a polyfill for the nightly [`std::panic::abort_unwind`] function.
//! The `#[noexcept]` attribute is implemented in terms of this function.
//!
//! On older versions, this requires use of [`libabort`] to abort the program.
//! This is necessary to support aborts in `#[no_std]` mode.
//! Enabling the `std` feature simply makes libabort delegate to [`std::process::abort`].
//!
//! [`libabort`]: https://github.com/Techcable/libabort.rs
//! [`std::panic::abort_unwind`]: https://doc.rust-lang.org/nightly/std/panic/fn.abort_unwind.html
//! [`noexcept` specifier]: https://en.cppreference.com/w/cpp/language/noexcept_spec.html
//! [`std::process::abort`]: https://doc.rust-lang.org/std/process/fn.abort.html
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

/// Indicates that a function should abort when panicking rather than unwinding.
///
/// This is equivalent to the C++ [`noexcept` specifier],
/// or the rustc-internal `#[rustc_nounwind]` attribute.
///
/// This is implemented using the [`nounwind::abort_unwind`](crate::abort_unwind) function.
///
/// [`noexcept` specifier]: https://en.cppreference.com/w/cpp/language/noexcept_spec.html
#[doc(inline)]
#[cfg(feature = "macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
pub use nounwind_macros::nounwind;

macro_rules! decl_abort_unwind {
    (
        $(#[$common_attr:meta])*
        pub fn abort_unwind(...);
    ) => {
        #[rustversion::since(1.81)]
        $(#[$common_attr])*
        pub extern "C" fn abort_unwind<F: FnOnce() -> R, R>(func: F) -> R {
            func()
        }

        #[rustversion::before(1.81)]
        $(#[$common_attr])*
        pub fn abort_unwind<F: FnOnce() -> R, R>(func: F) -> R {
            let guard = libabort::AbortGuard::new();
            let res = func();
            guard.defuse();
            res
        }
    }
}

decl_abort_unwind! {
    /// Invokes a closure, aborting if the closure unwinds.
    ///
    /// This is equivalent to the nightly-only [`std::panic::abort_unwind`] function.
    ///
    /// Where possible, this will include a panic message like "panic in a function that cannot unwind".
    /// However, this sort of message is not guaranteed.
    ///
    /// On older versions, this will fall back to using [`libabort`](https://github.com/Techcable/libabort.rs).
    ///
    /// [`std::panic::abort_unwind`]: https://doc.rust-lang.org/nightly/std/panic/fn.abort_unwind.html
    #[inline(always)]
    pub fn abort_unwind(...);
}
