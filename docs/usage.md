---
title: Usage
weight: 4
---

<a id="creating-a-temporary-directory"></a>

## Creating a temporary directory

To create a temporary directory simply call the `create` method on a `TemporaryDirectory` object.

```rust
# fn main() -> Result<(), spatie_temporary_directory::Error> {
use spatie_temporary_directory::TemporaryDirectory;

let temporary_directory = TemporaryDirectory::new().create()?;
# temporary_directory.delete()?;
# Ok(())
# }
```

Alternatively, use the static `make` method on a `TemporaryDirectory` object.

```rust
# fn main() -> Result<(), spatie_temporary_directory::Error> {
use spatie_temporary_directory::TemporaryDirectory;

let temporary_directory = TemporaryDirectory::make()?;
# temporary_directory.delete()?;
# Ok(())
# }
```

By default, the temporary directory will be created in your system's temporary directory (usually `/tmp`), with an automatically generated name.

## Naming your temporary directory

If you want to use a custom name for your temporary directory instead of the generated name call the `name` method with a string `name` argument before the `create` method.

```rust
# fn main() -> Result<(), spatie_temporary_directory::Error> {
# let parent = tempfile::tempdir()?;
# let name = format!("{}-example", parent.path().file_name().unwrap().to_string_lossy());
use spatie_temporary_directory::TemporaryDirectory;

let temporary_directory = TemporaryDirectory::new()
    .name(name)?
    .create()?;
# temporary_directory.delete()?;
# Ok(())
# }
```

By default an error will be returned if a directory already exists with the given argument. You can override this behaviour by calling the `force` method in combination with the `name` method.

```rust
# fn main() -> Result<(), spatie_temporary_directory::Error> {
# let parent = tempfile::tempdir()?;
# let name = format!("{}-example", parent.path().file_name().unwrap().to_string_lossy());
use spatie_temporary_directory::TemporaryDirectory;

let temporary_directory = TemporaryDirectory::new()
    .name(name)?
    .force()
    .create()?;
# temporary_directory.delete()?;
# Ok(())
# }
```

This deletes the existing entry and all of its contents, so use only a trusted parent and name. Names are trimmed; reserved characters, `.` and `..` are rejected.

## Setting a custom location for a temporary directory

You can set a custom location in which your temporary directory will be created by calling the `location` method with a `location` argument.

```rust
# fn main() -> Result<(), spatie_temporary_directory::Error> {
# let parent = tempfile::tempdir()?;
# let location = parent.path();
use spatie_temporary_directory::TemporaryDirectory;

let temporary_directory = TemporaryDirectory::new()
    .location(location)
    .create()?;
# temporary_directory.delete()?;
# Ok(())
# }
```

`TemporaryDirectory::make()` creates a directory in the system's temporary directory. Use `location()` to choose a different parent.

## Setting custom permissions for a temporary directory

By default, the temporary directory will be created with permissions `0o777`, filtered by the process's umask on Unix. You can customize these permissions by calling the `permission` method with an integer `permission` argument before the `create` method.

```rust
# fn main() -> Result<(), spatie_temporary_directory::Error> {
use spatie_temporary_directory::TemporaryDirectory;

let temporary_directory = TemporaryDirectory::new()
    .permission(0o755)
    .create()?;
# temporary_directory.delete()?;
# Ok(())
# }
```

## Determining paths within the temporary directory

You can use the `path_for` method to determine the full path to a file or directory in the temporary directory:

```rust
# fn main() -> Result<(), spatie_temporary_directory::Error> {
use spatie_temporary_directory::TemporaryDirectory;

let temporary_directory = TemporaryDirectory::new().create()?;
temporary_directory.path_for("dumps/datadump.dat")?;
# assert!(temporary_directory.path().join("dumps").is_dir());
# temporary_directory.delete()?;
# Ok(())
# }
```

`path()` returns the root. `path_for()` creates parent directories and returns a `PathBuf`. Child paths containing a dot are treated as filenames; paths without dots are directories. See [path handling details](#path-handling-details) for extensionless files and directories with dots in their names.

## Emptying a temporary directory

Use the `empty` method to delete all the files inside the temporary directory.

```rust
# fn main() -> Result<(), spatie_temporary_directory::Error> {
# let temporary_directory = spatie_temporary_directory::TemporaryDirectory::make()?.delete_when_destroyed(true);
# std::fs::write(temporary_directory.path_for("example.txt")?, "example")?;
temporary_directory.empty()?;
# assert_eq!(std::fs::read_dir(temporary_directory.path())?.count(), 0);
# Ok(())
# }
```

## Deleting a temporary directory

Once you're done processing your temporary data you can delete the entire temporary directory using the `delete` method. All files inside of it will be deleted.

```rust
# fn main() -> Result<(), spatie_temporary_directory::Error> {
# let temporary_directory = spatie_temporary_directory::TemporaryDirectory::make()?.delete_when_destroyed(true);
temporary_directory.delete()?;
# assert!(!temporary_directory.exists());
# Ok(())
# }
```

<a id="deleting-a-temporary-directory-when-the-object-is-destroyed"></a>

## Deleting a temporary directory when the object is destroyed

If you want to automatically have the filesystem directory deleted when the object instance goes out of scope, you can enable `delete_when_destroyed(true)` on the `TemporaryDirectory` object.

```rust
# fn main() -> Result<(), spatie_temporary_directory::Error> {
use spatie_temporary_directory::{Error, TemporaryDirectory};

fn handle_temporary_files() -> Result<(), Error> {
    let temporary_directory = TemporaryDirectory::new()
        .delete_when_destroyed(true)
        .create()?;

    // ... use the temporary directory

    Ok(()) // No need to manually call temporary_directory.delete()!
}

handle_temporary_files()?;
# Ok(())
# }
```

You can also call `drop(temporary_directory)`. Automatic cleanup is off by default; pass `false` to disable it. Drop cannot return errors, so use explicit `delete()` when a cleanup failure must be handled. Process termination and `mem::forget` can bypass Drop; it is not secret erasure.

## Directory lifecycle details

`new()` configures the object; `create()` performs filesystem I/O and consumes the configuration, returning the created directory. `make()` is shorthand for `new().create()` and does not enable automatic cleanup. Keep the returned object alive while its files are needed. Keeping a `PathBuf` alone does not keep the directory alive.

Configure name and location before creation. Missing parents are created but are not owned or removed by the package. An empty location selects `std::env::temp_dir()`; prefer an absolute parent if your application may change its working directory. Changing `name` or `location` after creation changes explicit operations' paths without moving files. Drop still targets the last successfully created path. Create another object for another directory.

`empty()` requires creation first, recreates the root and returns `&Self` for chaining. It is destructive and not atomic: recreation can fail after the old contents are gone. `delete()` does not consume the object and succeeds for an absent directory. It removes symlinks, including broken ones, without following their targets. Close open files before deletion, especially on Windows. An object configured with a name can explicitly delete that path without `create()`; only delete paths you own. Automatic cleanup only removes a path this object successfully created.

Use `exists()` to check whether the configured path exists and `get_name()` to get its name.

<a id="path-handling-details"></a>

## Path handling details

`path_for` strips leading and trailing `/`, rejects parent components and existing symlink components, and checks dots in the child path only. A dot in the parent location does not change whether a child is treated as a file. For an extensionless file or a dotted directory, use `path().join(...)` and the standard filesystem API:

```rust
# fn main() -> Result<(), spatie_temporary_directory::Error> {
# let temporary_directory = spatie_temporary_directory::TemporaryDirectory::make()?.delete_when_destroyed(true);
std::fs::write(temporary_directory.path().join("LICENSE"), "extensionless file")?;
std::fs::create_dir_all(temporary_directory.path().join("cache.v2"))?;
# Ok(())
# }
```

A plain `PathBuf::join` does not validate containment. Only join trusted relative paths; do not use it to bypass a rejected `path_for` call. Child-path checks do not protect against another process changing paths between checks and use.

<a id="handling-errors"></a>

## Handling errors

Methods that can fail return `spatie_temporary_directory::Error`. Use `?` to propagate the error, or match a variant when your application can recover.

| Error | Meaning | What to do |
| --- | --- | --- |
| `InvalidDirectoryName` | The name contains reserved characters or is `.` or `..` | Choose a single safe name, not a path |
| `PathAlreadyExists` | Another entry already occupies the requested path | Use a generated name, or explicitly opt into destructive `.force()` |
| `InvalidChildPath` | A child escapes the root or contains an existing symlink | Reject the input; do not bypass validation |
| `NotCreated` | `path_for` or `empty` ran before `create` | Create the directory first |
| `Io` | An operating system operation failed | Inspect the wrapped `std::io::Error`, including its `kind()` |

<a id="security-and-platform-limits"></a>

## Security and platform limits

- Use a parent controlled by your application. Path checks cannot stop another process replacing ancestors or symlinks between validation and use.
- Generated names avoid ordinary collisions but are not cryptographically unpredictable.
- `.force()` removes existing files, directories or symlinks without checking ownership. Never combine it with untrusted names or locations.
- Use `0o700` for private directories on Unix, subject to umask. The configured mode also applies to new child directories and the root recreated by `empty()`, not existing parents. Windows inherits ACLs instead.
- Recursive deletion removes entries, not file contents from storage. It is not secure erasure.
- Drop is best-effort cleanup, not a guarantee across crashes, aborts or process termination. Read bytes or move output elsewhere before dropping the owner.
