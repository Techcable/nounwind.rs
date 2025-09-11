# threadid.rs
<!-- cargo-rdme start -->

Defines a `#[nounwind]` attribute macro that prevents panics from unwinding,
similar to the C++ [`noexcept` specifier].

This is clearer than using a drop guard,
and in some versions of Rust can provide a better error message.
In particular, aborting panics will often print a messages like "panic in a function that cannot unwind",
although this behavior is version-specific and not guaranteed to occur.

The crate also provides a polyfill for the nightly [`std::panic::abort_unwind`] function.
The `#[noexcept]` attribute is implemented in terms of this function.

On older versions, this requires use of [`libabort`] to abort the program.
This is necessary to support aborts in `#[no_std]` mode.
Enabling the `std` feature simply makes libabort delegate to [`std::process::abort`].

[`libabort`]: https://github.com/Techcable/libabort.rs
[`std::panic::abort_unwind`]: https://doc.rust-lang.org/nightly/std/panic/fn.abort_unwind.html
[`noexcept` specifier]: https://en.cppreference.com/w/cpp/language/noexcept_spec.html
[`std::process::abort`]: https://doc.rust-lang.org/std/process/fn.abort.html

<!-- cargo-rdme end -->

## License
Licensed under either the [Apache 2.0 License](./LICENSE-APACHE.txt) or [MIT License](./LICENSE-MIT.txt) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
