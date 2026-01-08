test: _check && check-format
    cargo nextest run
    cargo test --doc

check: _check lint

lint: && check-format
    cargo rdme --check
    lychee README.md

_check:
    @-just spellcheck
    cargo doc --workspace --no-deps --document-private-items
    cargo clippy --workspace --all-targets

export RUST_LOG := "taplo:warn"
format: && _check_sorting spellcheck
    taplo format
    cargo +nightly fmt

check-format: && _check_sorting spellcheck
    taplo format --check
    cargo +nightly fmt --check

_check_sorting:
    cargo sort --grouped --workspace --check . >/dev/null

spellcheck:
    typos
    git log | typos -

fix-spelling:
    typos --write-changes
    git log | typos -
