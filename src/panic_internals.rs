//! Internals for the [`crate::panic_nounwind!`] macro and friends.

#[cold]
#[inline(never)]
#[track_caller]
pub fn unreachable_nounwind() -> ! {
    crate::panic_nounwind("internal error: entered unreachable code")
}

/// Calls `panic!` with the specified message, but guaranteed to abort instead of unwinding.
///
/// This is an implementation detail of the [`crate::panic_nounwind!`] macro,
/// and is not part of the crate's public API.
/// As such, it is exempt from semver guarantees.
///
/// This mirrors the [`core::panicking::panic_nounwind_fmt`] function in the standard library,
/// but without the parameter controlling backtrace suppression.
///
/// [`core::panicking::panic_nounwind_fmt`]: https://github.com/rust-lang/rust/blob/1.92.0/library/core/src/panicking.rs#L83-L95
#[inline(never)]
#[cold]
pub fn panic_nounwind_fmt(f: core::fmt::Arguments<'_>) -> ! {
    // This gives a better error message than using abort_unwind.
    // That prints two panic messages: First the real panic message,
    // and second a "panic in a function which can't unwind".
    // Even worse, the second message always includes a backtrace
    // unrelated to the real backtrace.
    //
    // TODO: Take advantage of libabort or something like it to provide these better messages on #[no_std]
    #[cfg(feature = "std")]
    {
        let _guard = crate::abort_guard::AbortGuard;
        panic!("{}", f)
    }
    #[cfg(not(feature = "std"))]
    {
        crate::abort_unwind(|| panic!("{}", f))
    }
}
