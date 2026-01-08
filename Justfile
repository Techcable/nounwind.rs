
format: && spellcheck
    taplo format
    cargo +nightly fmt

check: check-format
    -just spellcheck
    cargo clippy --workspace --all-targets
    cargo doc --workspace --no-deps --document-private-items
    cargo rdme --check
    lychee README.md

check-format: && spellcheck
    taplo format
    cargo +nightly fmt --check
    cargo sort --grouped --workspace --check . >/dev/null

spellcheck:
    typos
    git log | typos -

fix-spelling:
    typos --write-changes
    git log | typos -
