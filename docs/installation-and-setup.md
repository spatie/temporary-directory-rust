---
title: Installation
weight: 3
---

You can install the package via Cargo. Add this Git dependency to your application's `Cargo.toml`:

```toml
[dependencies]
spatie-temporary-directory = { git = "https://github.com/spatie/temporary-directory-rust", branch = "main" }
```

Requires Rust 1.89 or newer. The crate is not yet published to crates.io. Commit your application's `Cargo.lock` to keep the selected Git revision between builds. You can install Rust with [rustup](https://rustup.rs/). No PHP runtime or external programs are required.
