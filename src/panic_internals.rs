//! Internals for the [`crate::panic_nounwind!`] macro and friends.

#[cold]
#[inline(never)]
#[track_caller]
pub fn unreachable_nounwind() -> ! {
    crate::panic_nounwind("internal error: entered unreachable code")
}

/// Implementation detail of the [`crate::panic_nounwind!`] macro,
/// used to optimize for constant strings.
///
/// Needs to be a sepearete function rather than part of the macro
/// due to lifetime temporary extension issues on versions before Rust 1.89.
///
/// # Purpose
/// This optimization benefits constant messages by avoiding expesnive formatting machinery.
/// In particular this means `panic_nounwind!("msg")` will lower to `panic_nounwind("msg")`,
/// eliminating the `panic_nounwind_fmt` call which requires much more code to invoke.
///
/// In theory, this branch can have a slight runtime and code-size cost
/// if the direction of cannot be statically determined.
/// In practice, the direction of the branch and the value of the message
/// are always resolved at compile time.
/// This fact is relied upon by the implementation of `fmt::Arguments::as_statically_known_str`,
/// which just tests `llvm.isconstant` on the result of as `as_str`.
/// Even in the unlikely case the `as_str` cannot be resolved at compile time,
/// the cold path is so much more costly in code-size that this is worth the risk.
//
/// This also trusts `fmt::Arguments::as_str` never to panic,
/// which is true on every supported rust version.
/// We could use [`crate::abort_unwind`] to ensure this is true.
/// This has no runtime cost after the inlining,
/// but would harm compile times.
///
/// ## Inlining
/// Use of `inline(always)` is not helpful in debug builds.
/// Without optimizations, the `Arguments::as_str` and `Arguments::new_v1` methods are not inlined,
/// preventing any constant-elimination.
/// Even with a plain `#[inline]` marker, `-Copt-level=1` is smart enough to figure everything out.
///
/// The reason we mark `inline(always)` is so that this is inlined
/// even when `-Copt-level=s` or the calling function is marked `#[cold]`.
/// In these cases, a plain `#[inline]` hint is not enough to trigger inlining this function.
/// Generally the optimization works as intended and actually reduces the code size,
/// so it makes sense to override the optimizer here.
///
/// The code size difference appears to be more significant on x86_64 than aarch64.
#[inline(always)]
#[track_caller]
pub fn do_panic_nounwind(args: core::fmt::Arguments<'_>) -> ! {
    if let Some(msg) = args.as_str() {
        crate::panic_nounwind(msg)
    } else {
        panic_nounwind_fmt(args)
    }
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
#[track_caller]
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
