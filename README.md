# threadid.rs
<!-- cargo-rdme start -->

Defines a `#[nounwind]` attribute macro that prevents panics from unwinding,
similar to the C++ [`noexcept` specifier].


The [`panic_nounwind!`] macro offers a version of [`core::panic!`] that is guaranteed to abort instead of unwinding.
This is useful for fatal errors which cannot possibly be recovered from.
In particular, if proceeding could cause undefined behavior,
[`panic_nounwind!`] should be used instead of [`core::panic!`].

The crate also provides a polyfill for the nightly [`std::panic::abort_unwind`] function.
This provides more detailed control over what sections of code can and cannot panic.
It can also be used as a replacement to `#[nounwind]` if you want to avoid a macro dependency.

Using `#[nounwind]` is clearer than using a drop guard,
and in some versions of Rust can provide a better error message.
In particular, on recent versions of rust using `#[nounwind]` will print a messages like "panic in a function that cannot unwind".

Using [`panic_nounwind!`] is preferable to `abort_unwind(|| panic!(..))`, for reasons described in the [`abort_unwind`] docs.

## Feature Flags
The `std` feature provides superior error messages, so should be enabled wherever possible.

If the `std` feature cannot be enabled, and supporting versions of rust before 1.81 is needed,
enable the `old-rust-nostd` feature.
This will use [`libabort`] to provide a polyfill for [`std::process::abort`].

[`libabort`]: https://github.com/Techcable/libabort.rs
[`std::panic::abort_unwind`]: https://doc.rust-lang.org/nightly/std/panic/fn.abort_unwind.html
[`noexcept` specifier]: https://en.cppreference.com/w/cpp/language/noexcept_spec.html
[`std::process::abort`]: https://doc.rust-lang.org/std/process/fn.abort.html

<!-- cargo-rdme end -->

[`panic_nounwind!`]: https://docs.rs/nounwind/latest/nounwind/macro.panic_nounwind.html
[`core::panic!`]: https://doc.rust-lang.org/core/macro.panic.html
[`abort_unwind`]: https://docs.rs/nounwind/latest/nounwind/fn.abort_unwind.html

## License
Licensed under either the [Apache 2.0 License](./LICENSE-APACHE.txt) or [MIT License](./LICENSE-MIT.txt) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
