# Changelog

Notable changes to this project should be documented in this file.
Make sure it is up to date before performing a release.

This project follows the [Keep a Changelog](https://keepachangelog.com/en/2.0.0/) format wherever that is reasonable.

The "title" of each release should be its first line.
A title is required for publishing a github release, so all versions should have one.

Most changes include the relevant [jj](https://jj-vcs.dev) change ids in parens. An example of a change id is wuoxvnsw.

Old versions (before v0.1.6) were retroactively given CHANGELOG entries based on Github Releases.

## Unreleased

### Changed
- Do not track `Cargo.lock` in version control (yqxpqkkz)
- Switch from cargo-rdme to cargo-reedme (rqzwrlnq)

## 0.1.5
Add `#[track_caller]` to internal `panic_nounwind_fmt`, fixing `panic_nounwind!` location info.

### Fixed
- Add `#[track_caller]` to internal function `panic_nounwind_fmt`.
  Before this, the location information from `panic_nounwind!` (added in v0.1.3) was never actually being propagated.

No changes to `nounwind-macros` since v0.1.1.

## 0.1.4
Optimize `panic_nounwind!` with constant message.

### Added
- Adds doctests and a basic integration test.
  It's difficult to test the panic/abort case due to aborting the test process.

### Changed
- Optimizes `panic_nounwind!` and friends to avoid formatting overhead when the message is constant.
  This means `panic_nounwind!("message")` does not construct a `core::fmt::Arguments`,
  reducing code-size in the caller.
- Move panic internals to a separate (private) module to make it less likely to invoke accidentally.

No changes to `nounwind-macros` since v0.1.1.

## 0.1.3
Add `assert_nounwind!` macro

### Added
- The `assert_nounwind!` and `unreachable_nounwind!` macros are new convenience wrappers around `panic_nounwind!`.
  - I don't intend to add counterparts for the other panic macros, as they are much less useful.

### Changed
- Add `#[track_caller]` to the `nounwind::panic_nounwind()` function.
  - This has a minor increase in code size in the caller, but this can easily be avoided as described in the docs.
  - *NOTE*: This functionality was broken before the v0.1.5 release.

No changes to `nounwind-macros` since v0.1.1.

## 0.1.2
Add `panic_nounwind!` macro.

### Added
The newly added `panic_nounwind!` provides superior error messages to `nounwind::abort_unwind(|| panic!())`.
The latter prints two panic messages and a confusing backtrace.
These improved error messages require the `std` feature to be enabled.

In the next semver-breaking version of `nounwind` (v0.2.0), the `std` feature will be enabled by default (issue [#2]).

[#2]: https://github.com/Techcable/nounwind.rs/issues/2

### Changed
- Change the `libabort` feature to be an optional dependency enabled by the `old-rust-nostd` feature (on by default).
  - If the `std` feature is enabled, old versions of Rust can still be supported without needing `old-rust-nostd`.
- Change MSRV from v1.61 to v1.56 (uzkqqyyt)
  - The `nounwind-macros` crate still requires v1.61

No changes to `nounwind-macros` since v0.1.1.

*NOTE*: This version of `nounwind` was mistakenly released as `nounwind` v0.1.1.
I later yanked it when I realized I already released `nounwind-macros` v0.1.1.


## 0.1.1
Use `syn-mid` crate for macros instead of `syn/full`

The `nounwind-macros` crate does not need to parse the body of the function, so we can use the [taiki-e/syn-mid] crate instead of syn/full. This should reduce compile times (at least as long as no other crate requires syn/full).

[taiki-e/syn-mid]: https://github.com/taiki-e/syn-mid

## 0.1.0
Initial release.

Passes basic tests. It should be useful!

Supports Rust 1.61.
