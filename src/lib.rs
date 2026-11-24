#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Failures configuring or using a temporary directory.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A directory name contains reserved characters or names a parent directory.
    #[error("The directory name `{0}` contains invalid characters.")]
    InvalidDirectoryName(String),
    /// The requested path already exists and `force` was not enabled.
    #[error("Path `{0}` already exists.")]
    PathAlreadyExists(PathBuf),
    /// A child path would leave the temporary directory.
    #[error("Path `{0}` must stay inside the temporary directory.")]
    InvalidChildPath(PathBuf),
    /// The operation needs a directory that has been created.
    #[error("Create the temporary directory first.")]
    NotCreated,
    /// An operating system operation failed.
    #[error(transparent)]
    Io(#[from] io::Error),
}

/// A configurable temporary directory with opt-in deletion on drop.
///
/// Configuration methods consume and return the object. Filesystem operations
/// return `Result` so callers can inspect cleanup failures.
#[derive(Debug)]
pub struct TemporaryDirectory {
    location: PathBuf,
    name: String,
    force_create: bool,
    delete_when_destroyed: bool,
    permission: u32,
    created_path: Option<PathBuf>,
}

impl Default for TemporaryDirectory {
    fn default() -> Self {
        Self::new()
    }
}

impl TemporaryDirectory {
    /// Configure a directory under the operating system's temporary directory.
    pub fn new() -> Self {
        Self {
            location: std::env::temp_dir(),
            name: String::new(),
            force_create: false,
            delete_when_destroyed: false,
            permission: 0o777,
            created_path: None,
        }
    }

    /// Create a directory with default configuration.
    pub fn make() -> Result<Self, Error> {
        Self::new().create()
    }

    /// Set a parent directory. An empty path selects the system temporary directory.
    pub fn location(mut self, location: impl AsRef<Path>) -> Self {
        self.location = if location.as_ref().as_os_str().is_empty() {
            std::env::temp_dir()
        } else {
            location.as_ref().to_path_buf()
        };
        self
    }

    /// Set a directory name; an empty name requests an automatically generated name.
    pub fn name(mut self, name: impl AsRef<str>) -> Result<Self, Error> {
        let name = name.as_ref().trim();
        if matches!(name, "." | "..") || name.chars().any(|c| "\\/?%*:|\"<>\0".contains(c)) {
            return Err(Error::InvalidDirectoryName(name.into()));
        }
        self.name = name.into();
        Ok(self)
    }

    /// Replace an existing named directory and its contents when creating it.
    pub fn force(mut self) -> Self {
        self.force_create = true;
        self
    }

    /// Set Unix creation permissions, filtered by the process umask.
    ///
    /// Windows uses the inherited directory ACL instead.
    pub fn permission(mut self, permission: u32) -> Self {
        self.permission = permission;
        self
    }

    /// Enable or disable best-effort cleanup when the object is dropped.
    ///
    /// Use [`Self::delete`] for observable cleanup errors.
    pub fn delete_when_destroyed(mut self, enabled: bool) -> Self {
        self.delete_when_destroyed = enabled;
        self
    }

    /// Create the directory, including missing parent directories.
    pub fn create(mut self) -> Result<Self, Error> {
        if self.name.is_empty() {
            static SEQUENCE: AtomicU64 = AtomicU64::new(0);
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default();
            self.name = format!(
                "{}-{}-{}",
                std::process::id(),
                time.as_nanos(),
                SEQUENCE.fetch_add(1, Ordering::Relaxed)
            );
        }
        let path = self.path();
        if self.force_create {
            remove(&path)?;
        }
        self.mkdir(&self.location, true)?;
        match self.mkdir(&path, false) {
            Ok(()) => self.created_path = Some(path),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                return Err(Error::PathAlreadyExists(path));
            }
            Err(error) => return Err(error.into()),
        }
        Ok(self)
    }

    /// Return the root path without creating it.
    pub fn path(&self) -> PathBuf {
        self.location.join(&self.name)
    }

    /// Return a child path and create its directories.
    ///
    /// As in PHP, a path containing a dot is treated as a filename; otherwise the
    /// full path is created as a directory. Leading/trailing `/` are stripped.
    /// Parent components and existing symlink components are rejected.
    pub fn path_for(&self, child: impl AsRef<Path>) -> Result<PathBuf, Error> {
        self.require_created()?;
        let child = child.as_ref();
        let trimmed;
        let relative = if let Some(text) = child.to_str() {
            trimmed = text.trim_matches('/').to_owned();
            Path::new(&trimmed)
        } else {
            child
        };
        if relative.components().any(|c| {
            matches!(
                c,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        }) {
            return Err(Error::InvalidChildPath(child.into()));
        }
        let mut checked = self.path();
        if fs::symlink_metadata(&checked).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err(Error::InvalidChildPath(child.into()));
        }
        for component in relative.components() {
            checked.push(component);
            if fs::symlink_metadata(&checked).is_ok_and(|m| m.file_type().is_symlink()) {
                return Err(Error::InvalidChildPath(child.into()));
            }
        }
        let path = self.path().join(relative);
        let directory = if relative.as_os_str().as_encoded_bytes().contains(&b'.') {
            path.parent()
                .ok_or_else(|| Error::InvalidChildPath(child.into()))?
        } else {
            &path
        };
        self.mkdir(directory, true)?;
        Ok(path)
    }

    /// Return whether the configured path exists. Unnamed, uncreated objects return false.
    pub fn exists(&self) -> bool {
        !self.name.is_empty() && self.path().exists()
    }

    /// Return the configured or generated directory name.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Delete the contents and recreate the root with the configured permissions.
    pub fn empty(&self) -> Result<&Self, Error> {
        self.require_created()?;
        remove(&self.path())?;
        self.mkdir(&self.path(), true)?;
        Ok(self)
    }

    /// Recursively remove the directory without following symbolic links.
    ///
    /// Deleting an absent directory succeeds. An unnamed, uncreated object is a no-op.
    pub fn delete(&self) -> Result<(), Error> {
        if !self.name.is_empty() {
            remove(&self.path())?;
        }
        Ok(())
    }

    fn require_created(&self) -> Result<(), Error> {
        if self.created_path.is_none() {
            return Err(Error::NotCreated);
        }
        Ok(())
    }

    fn mkdir(&self, path: &Path, recursive: bool) -> io::Result<()> {
        let mut builder = fs::DirBuilder::new();
        builder.recursive(recursive);
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(self.permission);
        }
        builder.create(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        if self.delete_when_destroyed
            && let Some(path) = &self.created_path
        {
            let _ = remove(path);
        }
    }
}

fn remove(path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() => fs::remove_dir_all(path),
        Ok(metadata) => {
            #[cfg(windows)]
            {
                use std::os::windows::fs::FileTypeExt;
                if metadata.file_type().is_symlink_dir() {
                    return fs::remove_dir(path);
                }
            }
            let _ = metadata;
            fs::remove_file(path)
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(doctest)]
mod guide {
    #[doc = include_str!("../docs/usage.md")]
    mod usage {}
}
