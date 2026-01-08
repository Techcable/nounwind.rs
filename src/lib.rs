//! Defines a `#[nounwind]` attribute macro that prevents panics from unwinding,
//! similar to the C++ [`noexcept` specifier].
//!
//! The [`panic_nounwind!`] macro offers a version of [`core::panic!`] that is guaranteed to abort instead of unwinding.
//! This is useful for fatal errors which cannot possibly be recovered from.
//! In particular, if proceeding could cause undefined behavior,
//! [`panic_nounwind!`] should be used instead of [`core::panic!`].
//! Similar [`assert_nounwind!`] and [`unreachable_nounwind!`] macros are offered,
//! which are convenience wrappers around [`panic_nounwind!`].
//!
//! The crate also provides a polyfill for the nightly [`std::panic::abort_unwind`] function.
//! This provides more detailed control over what sections of code can and cannot panic.
//! It can also be used as a replacement to `#[nounwind]` if you want to avoid a macro dependency.
//!
//! Using `#[nounwind]` is clearer than using a drop guard,
//! and in some versions of Rust can provide a better error message.
//! In particular, on recent versions of rust using `#[nounwind]` will print a messages like "panic in a function that cannot unwind".
//!
//! Using [`panic_nounwind!`] is preferable to `abort_unwind(|| panic!(..))`, for reasons described in the [`abort_unwind`] docs.
//!
//! # Feature Flags
//! The `std` feature provides superior error messages, so should be enabled wherever possible.
//!
//! If the `std` feature cannot be enabled, and supporting versions of rust before 1.81 is needed,
//! enable the `old-rust-nostd` feature.
//! This will use [`libabort`] to provide a polyfill for [`std::process::abort`].
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
///
/// # Examples
/// ```
/// #[nounwind::nounwind]
/// fn print_nounwind(msg: &str) {
///     println!("{msg}");
/// }
/// print_nounwind("foo");
/// ```
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
            #[cfg(any(feature = "std", feature = "old-rust-nostd"))]
            let guard = abort_guard::AbortGuard;
            #[cfg(all(not(feature = "old-rust-nostd"), not(feature = "std")))]
            let guard = {
                compile_error!(
                    r#"Using the `nounwind` crate with this version of rust requires either `feature = "std"` or `feature = "old-rust-nostd"`"#
                );
                ()
            };
            let res = func();
            core::mem::forget(guard);
            res
        }
    }
}

#[cfg(any(feature = "std", feature = "old-rust-nostd"))]
mod abort_guard {
    #[allow(unused)]
    pub struct AbortGuard;
    impl Drop for AbortGuard {
        #[inline]
        fn drop(&mut self) {
            #[cfg(feature = "std")]
            std::process::abort();
            #[cfg(all(feature = "old-rust-nostd", not(feature = "std")))]
            libabort::abort();
        }
    }
}

decl_abort_unwind! {
    /// Invokes a closure, aborting if the closure unwinds.
    ///
    /// This is equivalent to the nightly-only [`std::panic::abort_unwind`] function.
    ///
    /// Prefer the [`panic_nounwind!`] macro to `abort_unwind(|| panic!(...))`,
    /// as the first gives a confusing error message.
    ///
    /// As of Rust 1.92, this will print a second panic message "panic in a function that cannot unwind".
    /// This is usually a desirable outcome, but also explains why `abort_unwind(|| panic!(user_msg))` gives a confusing message.
    /// The "panic in a function that cannot unwind" message is printed after `user_msg` is, obscuring the real panic message.
    /// To make matters worse, the second backtrace is enabled by default,
    /// whereas the first is disabled by default.
    /// This makes it even harder to notice the real error message.
    /// Using [`panic_nounwind!`] avoids that.
    ///
    /// On older versions of Rust, and when `feature = "std"` is not enabled,
    /// this will fall back to using [`libabort`](https://github.com/Techcable/libabort.rs).
    ///
    /// [`std::panic::abort_unwind`]: https://doc.rust-lang.org/nightly/std/panic/fn.abort_unwind.html
    ///
    /// # Examples
    /// ```
    /// fn print_nounwind(msg: &str) {
    ///     nounwind::abort_unwind(|| {
    ///         println!("{msg}");
    ///     });
    /// }
    /// print_nounwind("foo");
    /// ```
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
///
/// # Examples
/// To avoid some overhead in the caller,
/// it is possible to extract to a helper function and omit `#[track_caller]`.
/// In this case, the caller will only need a single jump instruction.
/// ```
/// #[inline]
/// fn increment(x: u32) -> u32 {
///     x.checked_add(1).unwrap_or_else(|| increment_failure())
/// }
/// #[inline(never)]
/// #[cold]
/// fn increment_failure() -> ! {
///     nounwind::panic_nounwind!("Unrecoverable Error: Failed to increment integer")
/// }
/// increment(7); // will succeed
/// increment(8); // will succeed
/// ```
///
/// The standard formatting options are available:
/// ```no_run
/// # use nounwind::panic_nounwind;
/// panic_nounwind!("hello"); // prints "hello"
/// let x = 7;
/// panic_nounwind!("hello {x}"); // prints "hello 7"
/// panic_nounwind!("hello {{}}"); // prints "hello {}"
/// ```
#[macro_export]
macro_rules! panic_nounwind {
    ($($arg:tt)*) => ($crate::panic_nounwind_fmt(format_args!($($arg)*)));
}

/// Equivalent to [`core::assert!`], but guaranteed to abort the program instead of unwinding.
///
/// This function is useful for checking invalid state which cannot possibly be repaired.
/// In particular, this is more appropriate than [`core::assert!`] for checking soundess errors.
/// See the [`panic_nounwind!`] macro and [`unreachable_nounwind!`] for details.
///
/// # Examples
/// ```
/// nounwind::assert_nounwind!(3 + 7 > 2); // would print "assertion failed: 3 + 7 > 2"
/// nounwind::assert_nounwind!(3 + 7 > 2, "message"); // would print "message"
/// let x = 7;
/// nounwind::assert_nounwind!(3 + 7 > 2, "message {x}"); // would print "message 7"
/// nounwind::assert_nounwind!(3 + 7 > 2, "message {{}}"); // would print "message {}"
/// ```
#[macro_export]
macro_rules! assert_nounwind {
    ($cond:expr) => {
        if !($cond) {
            $crate::panic_nounwind(concat!("assertion failed: ", stringify!($cond)));
        };
    };
    ($cond:expr, $($arg:tt)+) => {
        if !($cond) {
            $crate::panic_nounwind_fmt(format_args!($($arg)*));
        }
    }
}

/// Equivalent to [`core::unreachable!`], but guaranteed to abort the program instead of unwinding.
///
/// This function is useful if it would be undefined behavior to continue.
/// See the [`panic_nounwind!`] macro for details.
///
/// # Examples
/// ```
/// use nounwind::unreachable_nounwind;
///
/// fn infallible() -> u32 {
///     match (3u32.checked_add(7)) {
///         Some(x) => x,
///         None => unreachable_nounwind!("addition failed"),
///     }
/// }
/// ```
///
/// Several formatting messages are possible:
/// ```no_run
/// # use nounwind::unreachable_nounwind;
/// unreachable_nounwind!(); // "internal error: entered unreachable code"
/// unreachable_nounwind!("foo"); // "internal error: entered unreachable code: foo"
/// unreachable_nounwind!("foo {}", 7); // "internal error: entered unreachable code: foo 7"
/// unreachable_nounwind!("foo {{}}"); // "internal error: entered unreachable code: foo {}"
/// ```
#[macro_export]
macro_rules! unreachable_nounwind {
    () => ($crate::unreachable_nounwind());
    ($($arg:tt)+) => {
        $crate::panic_nounwind!(
            "internal error: entered unreachable code: {}",
            format_args!($($arg)*)
        );
    }
}

#[cold]
#[inline(never)]
#[track_caller]
#[doc(hidden)]
pub fn unreachable_nounwind() -> ! {
    panic_nounwind("internal error: entered unreachable code")
}

/// Triggers a [`core::panic!`] with the specified message, but guaranteed to abort instead of unwinding.
///
/// See [`panic_nounwind!`] macro for examples and use cases.
///
/// This mirrors the [`core::panicking::panic_nounwind`] function in the standard library.
/// This is part of the `panic_internals` nightly feature,
/// and is used for fatal runtime errors inside of the stdlib.
///
/// This function preserves location information (it is marked with `#[track_caller]`).
/// This slightly increases code size in the caller,
/// which can avoided by outlining the panic call or switching to [`std::process::abort`].
///
/// [`core::panicking::panic_nounwind`]: https://github.com/rust-lang/rust/blob/1.92.0/library/core/src/panicking.rs#L222-L231
/// [`std::process::abort`]: https://doc.rust-lang.org/std/process/fn.abort.html
///
/// # Examples
/// ```no_run
///  # use nounwind::panic_nounwind;
/// panic_nounwind("goodbye world");
/// ```
#[cold]
#[inline(never)]
#[track_caller]
pub fn panic_nounwind(s: &'static str) -> ! {
    panic_nounwind_fmt(format_args!("{}", s))
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
    // This gives a better error message than using abort_unwind.
    // That rints two panic messages: First the real panic message,
    // and second a "panic in a function which can't unwind".
    // Even worse, the second message always includes a backtrace
    // unrelated to the real backtrace.
    //
    // TODO: Take advantage of libabort or something like it to provide these better messages on #[no_std]
    #[cfg(feature = "std")]
    {
        let _guard = abort_guard::AbortGuard;
        panic!("{}", f)
    }
    #[cfg(not(feature = "std"))]
    {
        crate::abort_unwind(|| panic!("{}", f))
    }
}
