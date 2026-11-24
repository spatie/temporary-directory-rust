# Quickly create, use and delete temporary directories

[![Latest version on crates.io](https://img.shields.io/crates/v/spatie-temporary-directory.svg?style=flat-square)](https://crates.io/crates/spatie-temporary-directory)
[![Tests](https://img.shields.io/github/actions/workflow/status/spatie/temporary-directory-rust/ci.yml?branch=main&style=flat-square)](https://github.com/spatie/temporary-directory-rust/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](https://github.com/spatie/temporary-directory-rust/blob/main/LICENSE.md)
[![Total downloads](https://img.shields.io/crates/d/spatie-temporary-directory.svg?style=flat-square)](https://crates.io/crates/spatie-temporary-directory)

This package allows you to quickly create, use and delete a temporary directory in the system's temporary directory.

Here's a quick example on how to create a temporary directory and delete it:

```rust
# fn main() -> Result<(), spatie_temporary_directory::Error> {
use spatie_temporary_directory::TemporaryDirectory;

let temporary_directory = TemporaryDirectory::new().create()?;

// Get a path inside the temporary directory
temporary_directory.path_for("temporaryfile.txt")?;

// Delete the temporary directory and all the files inside it
temporary_directory.delete()?;
# assert!(!temporary_directory.exists());
# Ok(())
# }
```

This is the Rust version of [spatie/temporary-directory](https://github.com/spatie/temporary-directory).

## Support us

[<img src="https://spatie.be/github-ad-image/temporary-directory.jpg" alt="Support Spatie's open source work" width="419px" />](https://spatie.be/github-ad-click/temporary-directory)

We invest a lot of resources into creating [best in class open source packages](https://spatie.be/open-source). You can support us by [buying one of our paid products](https://spatie.be/open-source/support-us).

We highly appreciate you sending us a postcard from your hometown, mentioning which of our package(s) you are using. You'll find our address on [our contact page](https://spatie.be/about-us). We publish all received postcards on [our virtual postcard wall](https://spatie.be/open-source/postcards).

## Installation

You can install the package via Cargo. Add this Git dependency to your application's `Cargo.toml`:

```toml
[dependencies]
spatie-temporary-directory = { git = "https://github.com/spatie/temporary-directory-rust", branch = "main" }
```

Requires Rust 1.89 or newer. The crate is not yet published to crates.io. Commit your application's `Cargo.lock` to keep the selected Git revision between builds. No PHP runtime or external programs are required.

## Usage

To create a temporary directory, call `TemporaryDirectory::new().create()?`, or use the shorthand `TemporaryDirectory::make()?`. Use `name`, `location` and `permission` before creation to customize the directory. The [usage guide](docs/usage.md) explains each option with examples.

If you're coming from PHP, the [PHP and Rust API guide](PARITY.md) shows the corresponding method names and differences to keep in mind.

Automatic cleanup is **off by default**. Enable `.delete_when_destroyed(true)` before creation
to remove the directory at scope exit, including ordinary error returns. Call `delete()` explicitly
when you need to handle a cleanup error.

- [Installation and setup](docs/installation-and-setup.md)
- [Directory lifecycle and child paths](docs/usage.md#creating-a-temporary-directory)
- [Handling errors](docs/usage.md#handling-errors)
- [Security and platform limits](docs/usage.md#security-and-platform-limits)

## Testing

Run from this directory:

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo test --locked --doc
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps
```

Rustdoc compiles and runs every Rust code block in the README and guides. Setup and cleanup use hidden `# ` lines so the displayed examples stay close to the PHP documentation.

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## Contributing

See [CONTRIBUTING.md](https://github.com/spatie/temporary-directory-rust/blob/main/CONTRIBUTING.md) and the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

Using an AI coding agent? Load Apollo's [rust-best-practices skill](https://github.com/apollographql/skills/tree/main/skills/rust-best-practices) when writing or reviewing code in this package.

## Security Vulnerabilities

Report vulnerabilities privately using the instructions in [SECURITY.md](https://github.com/spatie/temporary-directory-rust/blob/main/SECURITY.md).
Temporary files are not a sandbox. Use a trusted parent directory and `permission(0o700)` for private data.
Do not let another process replace directories or symlinks while resolving child paths.

## Credits

Original PHP package by Alex Vanderbist and [contributors](https://github.com/spatie/temporary-directory/graphs/contributors).

- [Alex Vanderbist](https://github.com/AlexVanderbist)
- [All contributors](https://github.com/spatie/temporary-directory-rust/graphs/contributors)

## License

MIT. See [LICENSE.md](LICENSE.md).
