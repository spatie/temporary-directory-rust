# PHP and Rust API

If you've used [spatie/temporary-directory](https://github.com/spatie/temporary-directory), use this table to find the corresponding methods in Rust.

| PHP | Rust |
| --- | --- |
| `new TemporaryDirectory()` | `TemporaryDirectory::new()` / `Default::default()` |
| constructor location | `.location(path)` |
| `make()` | `TemporaryDirectory::make()` |
| `make(location)` | `TemporaryDirectory::new().location(path).create()?` |
| `create()` | `.create()?` |
| `force()` | `.force()` |
| `permission(mode)` | `.permission(mode)` |
| `name(name)` | `.name(name)?` |
| `location(location)` | `.location(path)` |
| `path()` | `.path()` |
| `path(child)` | `.path_for(child)?` |
| `empty()` | `.empty()?` |
| `delete()` | `.delete()?` |
| `exists()` | `.exists()` |
| `getName()` | `.get_name()` |
| `deleteWhenDestroyed(bool)` | `.delete_when_destroyed(bool)` |
| destructor | `Drop` |

Configuration methods consume and return the object, so keep the returned value or chain the calls together. Paths are `PathBuf` values and methods that can fail return `Result`. Use `?` to pass an error to your caller, or match `Error::InvalidDirectoryName` and `Error::PathAlreadyExists` when you want to handle those cases yourself.

## Differences to keep in mind

- Names cannot be `.` or `..`. Child paths cannot escape the root or pass through existing symlinks.
- Create the directory before calling `path_for` or `empty`. An unnamed object cannot delete the system's temporary root.
- `path_for` looks for dots in the child path to decide whether it names a file. A dot in the parent location does not affect that decision.
- Generated names have a different format from PHP and are not cryptographically unpredictable.
- Automatic cleanup remembers the last successfully created path. Changing `name` or `location` later does not change the cleanup target or move files. Create another object when you need another directory.

See [security and platform limits](docs/usage.md#security-and-platform-limits) before using temporary directories for private data or with untrusted paths.
