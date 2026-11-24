//! Directory lifecycle, path validation and platform-specific filesystem behavior.

use spatie_temporary_directory::{Error, TemporaryDirectory};
use std::fs;

#[test]
fn creates_unique_directories_with_shorthand_and_builder() -> Result<(), Box<dyn std::error::Error>>
{
    let first = TemporaryDirectory::make()?.delete_when_destroyed(true);
    let second = TemporaryDirectory::new()
        .delete_when_destroyed(true)
        .create()?;
    assert!(first.exists());
    assert!(second.exists());
    assert_ne!(first.path(), second.path());
    assert!(!first.get_name().contains(' '));
    Ok(())
}

#[test]
fn names_locations_force_and_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let parent = tempfile::tempdir()?;
    let directory = TemporaryDirectory::new()
        .location(parent.path().join("nested/"))
        .name("  named  ")?
        .delete_when_destroyed(true)
        .create()?;
    assert_eq!(directory.get_name(), "named");
    assert_eq!(directory.path(), parent.path().join("nested/named"));
    let child = directory.path_for("/abc/123/xyz/")?;
    assert!(child.is_dir());
    let file = directory.path_for("abc/123/data.txt")?;
    assert!(!file.exists());
    fs::write(&file, b"payload")?;
    assert!(matches!(
        TemporaryDirectory::new()
            .location(parent.path().join("nested"))
            .name("named")?
            .create(),
        Err(Error::PathAlreadyExists(_))
    ));
    let replacement = TemporaryDirectory::new()
        .location(parent.path().join("nested"))
        .name("named")?
        .force()
        .create()?;
    assert!(!file.exists());
    fs::write(replacement.path_for("second.txt")?, b"second")?;
    replacement.empty()?;
    assert!(replacement.exists());
    assert_eq!(fs::read_dir(replacement.path())?.count(), 0);
    replacement.delete()?;
    replacement.delete()?;
    assert!(!replacement.exists());
    Ok(())
}

#[test]
fn cleanup_is_opt_in_and_can_be_disabled() -> Result<(), Box<dyn std::error::Error>> {
    let parent = tempfile::tempdir()?;
    let persistent = TemporaryDirectory::new().location(parent.path()).create()?;
    let path = persistent.path();
    drop(persistent);
    assert!(path.is_dir());
    let temporary = TemporaryDirectory::new()
        .location(parent.path())
        .delete_when_destroyed(true)
        .create()?;
    let path = temporary.path();
    drop(temporary);
    assert!(!path.exists());
    let persistent = TemporaryDirectory::new()
        .location(parent.path())
        .delete_when_destroyed(true)
        .delete_when_destroyed(false)
        .create()?;
    let path = persistent.path();
    drop(persistent);
    assert!(path.is_dir());
    let owned = TemporaryDirectory::new()
        .location(parent.path())
        .delete_when_destroyed(true)
        .create()?;
    let owned_path = owned.path();
    let unrelated = parent.path().join("unrelated");
    fs::create_dir(&unrelated)?;
    let reconfigured = owned.name("unrelated")?;
    drop(reconfigured);
    assert!(!owned_path.exists());
    assert!(unrelated.is_dir());
    TemporaryDirectory::new().delete()?;
    Ok(())
}

#[test]
fn rejects_reserved_names_and_escaping_paths() -> Result<(), Box<dyn std::error::Error>> {
    for name in [
        "/", "a\\b", "?", "%", "*", ":", "|", "\"", "<", ">", ".", "..", "\0",
    ] {
        assert!(matches!(
            TemporaryDirectory::new().name(name),
            Err(Error::InvalidDirectoryName(_))
        ));
    }
    let directory = TemporaryDirectory::new()
        .delete_when_destroyed(true)
        .create()?;
    assert!(matches!(
        directory.path_for("../outside.txt"),
        Err(Error::InvalidChildPath(_))
    ));
    assert!(matches!(
        TemporaryDirectory::new().path_for("file.txt"),
        Err(Error::NotCreated)
    ));
    Ok(())
}

#[cfg(unix)]
#[test]
fn applies_permissions_to_root_and_children() -> Result<(), Box<dyn std::error::Error>> {
    use std::os::unix::fs::PermissionsExt;
    let directory = TemporaryDirectory::new()
        .permission(0o700)
        .delete_when_destroyed(true)
        .create()?;
    let child = directory.path_for("child")?;
    assert_eq!(
        fs::metadata(directory.path())?.permissions().mode() & 0o777,
        0o700
    );
    assert_eq!(fs::metadata(child)?.permissions().mode() & 0o777, 0o700);
    directory.empty()?;
    assert_eq!(
        fs::metadata(directory.path())?.permissions().mode() & 0o777,
        0o700
    );
    Ok(())
}

#[cfg(unix)]
#[test]
fn deletes_links_without_following_them() -> Result<(), Box<dyn std::error::Error>> {
    use std::os::unix::fs::symlink;
    let outside = tempfile::tempdir()?;
    fs::write(outside.path().join("keep.txt"), b"keep")?;
    let directory = TemporaryDirectory::new()
        .delete_when_destroyed(true)
        .create()?;
    symlink(outside.path(), directory.path().join("link"))?;
    symlink("missing", directory.path().join("broken"))?;
    assert!(matches!(
        directory.path_for("link/new.txt"),
        Err(Error::InvalidChildPath(_))
    ));
    directory.delete()?;
    symlink(outside.path(), directory.path())?;
    assert!(matches!(
        directory.path_for("new.txt"),
        Err(Error::InvalidChildPath(_))
    ));
    directory.delete()?;
    assert_eq!(fs::read(outside.path().join("keep.txt"))?, b"keep");
    Ok(())
}
