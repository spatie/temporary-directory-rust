# Quickly create, use and delete temporary directories

This package allows you to quickly create, use and delete a temporary directory in the system's temporary directory.

Here's a quick example on how to create a temporary directory and delete it:

```rust,ignore
use spatie_temporary_directory::TemporaryDirectory;

let temporary_directory = TemporaryDirectory::new().create()?;

// Get a path inside the temporary directory
temporary_directory.path_for("temporaryfile.txt")?;

// Delete the temporary directory and all the files inside it
temporary_directory.delete()?;
```

This is the Rust version of [spatie/temporary-directory](https://github.com/spatie/temporary-directory).

## Support us

Support our open source work by [buying one of our products](https://spatie.be/open-source/support-us).

## Installation

You can install the package via Cargo. Add this Git dependency to your application's `Cargo.toml`:

```toml
[dependencies]
spatie-temporary-directory = { git = "https://github.com/spatie/temporary-directory-rust", branch = "main" }
```

Requires Rust 1.89 or newer. The crate is not yet published to crates.io. Commit your application's `Cargo.lock` to keep the selected Git revision between builds. No PHP runtime or external programs are required.

## Usage

To create a temporary directory, call `TemporaryDirectory::new().create()?`, or use the shorthand `TemporaryDirectory::make()?`. Use `name`, `location` and `permission` before creation to customize the directory. The [usage guide](docs/usage.md) explains each option with examples.

If you're coming from PHP, the [PHP and Rust API guide](docs/parity.md) shows the corresponding method names and differences to keep in mind.

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

The `documentation` integration tests extract every Rust code block from the README and guides, compile it and run it. Setup supplies the existing directory, name and location outside the displayed examples, and removes test directories afterwards. The fragments use `rust,ignore` because they are executed by this target rather than by rustdoc without their application context.

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

## Contributing

See [Spatie's contribution guidelines](https://github.com/spatie/.github/blob/main/CONTRIBUTING.md).

## Security vulnerabilities

Report security problems to [security@spatie.be](mailto:security@spatie.be).
Temporary files are not a sandbox. Use a trusted parent directory and `permission(0o700)` for private data.
Do not let another process replace directories or symlinks while resolving child paths.

## Postcardware

If you use this package in production, we'd appreciate a postcard from your hometown.
Our address is Spatie, Kruikstraat 22, 2018 Antwerp, Belgium.

## Credits

Original PHP package by Alex Vanderbist and [contributors](https://github.com/spatie/temporary-directory/graphs/contributors).

## License

MIT. See [LICENSE.md](LICENSE.md).
