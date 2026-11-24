---
title: Testing
weight: 5
---

You can run the tests with Cargo:

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo test --locked --doc --all-features
cargo doc --locked --no-deps --all-features
cargo package --locked
```

CI runs these tests on Linux, macOS and Windows. It also checks Rust 1.89, rustfmt, Clippy and rustdoc. Unix-only tests check permissions and symlink behavior.

Set `RUSTFLAGS` and `RUSTDOCFLAGS` to `-D warnings` to match CI. Check the minimum supported Rust version with `cargo +1.89.0 test --locked --all-targets --all-features` and `cargo +1.89.0 test --locked --doc --all-features`.

Install `cargo-audit` with `cargo install cargo-audit --locked`, then run `cargo audit`. CI checks security advisories on Linux.
