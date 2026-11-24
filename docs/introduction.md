---
title: Introduction
weight: 1
---

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

You can also have the directory [deleted automatically](usage.md#deleting-a-temporary-directory-when-the-object-is-destroyed) when the object goes out of scope. Automatic cleanup is off by default.
