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
        #[cfg(nounwind_extern_c_will_abort)]
        $(#[$common_attr])*
        pub extern "C" fn abort_unwind<F: FnOnce() -> R, R>(func: F) -> R {
            func()
        }

        #[cfg(not(nounwind_extern_c_will_abort))]
        $(#[$common_attr])*
        pub fn abort_unwind<F: FnOnce() -> R, R>(func: F) -> R {
            struct AbortGuard;
            impl Drop for AbortGuard {
                #[inline]
                fn drop(&mut self) {
                    #[cfg(feature = "std")]
                    std::process::abort();
                    #[cfg(all(feature = "old-rust-nostd", not(feature = "std")))]
                    libabort::abort();
                    #[cfg(all(not(feature = "old-rust-nostd"), not(feature = "std")))]
                    compile_error!(r#"Using the `nounwind` crate with this version of rust requires either `feature = "std"` or `feature = "old-rust-nostd"`"#);
                }
            }
            let guard = AbortGuard;
            let res = func();
            core::mem::forget(guard);
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

/// Equivalent to [`core::panic!`], but guaranteed to abort the program instead of unwinding.
///
/// This is useful for fatal errors, which cannot possibly be recovered from.
/// In particular, if proceeding could cause undefined behavior,
/// this should be used instead of [`core::panic!`].
/// Recovery from undefined behavior is definitionally impossible and unwinding would only worsen the problem.
///
/// This includes location information, just like [`core::panic!`] does.
/// The [`panic_nounwind()`] function discards this information, decreasing the impact on code size along with the error quality.
#[macro_export]
macro_rules! panic_nounwind {
    ($($arg:tt)*) => ($crate::panic_nounwind_fmt(format_args!($($arg)*)));
}

/// Triggers a [`core::panic!`] with the specified message, but guaranteed to abort instead of unwinding.
///
/// Discards location information to reduce code size in the caller (so does not have `#[track_caller]`).
/// Use the [`panic_nounwind!`] macro if location information is desired.
///
/// See [`panic_nounwind!`] macro for examples and use cases.
///
/// This mirrors the [`core::panicking::panic_nounwind`] function in the standard library.
/// This is part of the `panic_internals` nightly feature,
/// and is used for fatal runtime errors inside of the stdlib.
///
/// [`core::panicking::panic_nounwind`]: https://github.com/rust-lang/rust/blob/1.92.0/library/core/src/panicking.rs#L222-L231
#[cold]
#[inline(never)]
pub fn panic_nounwind(s: &'static str) -> ! {
    crate::abort_unwind(|| panic!("{s}"))
}

/// Calls `panic!` with the specified message, but guaranteed to abort instead of unwinding.
///
/// This is an implementation detail of the [`panic_nounwind!`] macro,
/// and is not part of the crate's public API.
/// As such, it is exempt from semver guarantees.
///
/// This mirrors the [`core::panicking::panic_nounwind_fmt`] function in the standard library,
/// but without the parameter controlling backtrace suppression.
///
/// [`core::panicking::panic_nounwind_fmt`]: https://github.com/rust-lang/rust/blob/1.92.0/library/core/src/panicking.rs#L83-L95
#[inline(never)]
#[cold]
#[doc(hidden)]
pub fn panic_nounwind_fmt(f: core::fmt::Arguments<'_>) -> ! {
    crate::abort_unwind(|| panic!("{f}"))
}
