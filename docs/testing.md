---
title: Testing
weight: 5
---

You can run the tests with Cargo:

```sh
cargo test --locked --all-targets
cargo test --locked --doc
```

CI runs these tests on Linux, macOS and Windows. It also checks Rust 1.89, rustfmt, Clippy and rustdoc. Unix-only tests check permissions and symlink behavior.
