---
title: PHP and Rust API
weight: 7
---

If you've used [spatie/temporary-directory](https://github.com/spatie/temporary-directory), see [PARITY.md](https://github.com/spatie/temporary-directory-rust/blob/main/PARITY.md) for the corresponding Rust methods and differences.

Configuration methods consume and return the object. Filesystem methods return `Result`, paths are `PathBuf` values, and automatic cleanup is opt-in with `delete_when_destroyed(true)`.

See [security and platform limits](usage.md#security-and-platform-limits) before using temporary directories for private data or with untrusted paths.
