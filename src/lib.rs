#![cfg_attr(feature = "nightly", feature(try_from))]

//! Ephemeral creates a temporary project on your filesystem at any location of your choice
//! so that you can use it while testing anything that works on a rust project - mainly cargo
//! commands/binaries. It can be used to generate projects of other languages too.
//!
//! # INSTALLATION:
//!
//! To use this crate, add it to the dev-dependencies since it is used only during testing:
//!
//! ```toml
//! [dev-dependencies]
//! ephemeral = "0.2"
//! ```
//!
//! # USAGE:
//!
//! To create a project:
//!
//! ```rust
//! use ephemeral::{builder::{RustBuilder, Builder}, Dir };
//!
//! fn main() {
//!     let project = RustBuilder::new("tmp")
//!        .add_dir(Dir::new("tmp/foo").add_file("bar", &vec![101u8]))
//!        .build()
//!        .expect("cannot create project");
//!
//!     project.clear();
//! }
//! ```
//!
//! This will create a new project in a dir called `tmp` which will contain a dir "foo" which will
//! contain a file `bar` with `e` (101u8) written to the file.

use std::fs::{create_dir_all, remove_dir_all};
use std::{error::Error, path::PathBuf};

pub mod builder;
pub mod rust_tools;

/// Project represents a project created on the file system at any user-defined location defined by
/// the path parameter to the `new()` function.
///
/// This struct as a builder so directories and files can be added to it. Remember to call `build()`
/// at the end to create the project in the filesystem. The dirs vector will contain all the dirs &
/// subdirs in the project, which are added when the directory is added to the project.

#[derive(Clone, Debug)]
pub struct Project {
    pub path: PathBuf,
    dirs: Vec<Dir>,
}

impl Project {
    /// Creates a new Project at the specified `path`. This will automatically add a "root" directory
    /// to the `dirs` vector.

    pub fn new<T>(path: T) -> Project
    where
        T: Into<PathBuf> + Clone,
    {
        let path = path.into();
        Project {
            dirs: vec![Dir::new(&path)],
            path,
        }
    }

    /// Deletes the project from the filesystem. This function can be used to clear the project
    /// after running the tests.
    ///
    /// This function panics if a directory cannot be deleted.

    pub fn clear(self) {
        remove_dir_all(&self.dirs[0].path).expect("can't delete directory")
    }
}

/// Represents a dir in the filesystem. Accepts a path and contains a vector of files added.
///
/// To a Dir, you can attach files but not other dirs. To attach subdirectories, add them
/// directly to Project and specify the parent dir in the path.

#[derive(Clone, Debug)]
pub struct Dir {
    pub path: PathBuf,
    files: Vec<File>,
}

impl Dir {
    pub fn new<T: Into<PathBuf>>(path: T) -> Dir {
        Dir {
            path: path.into(),
            files: vec![],
        }
    }

    /// Adds a file to the Dir. Accepts any type that can be converted to a PathBuf just like the
    /// rest of the crate. Contents of the file should be specified as well (in bytes).

    pub fn add_file<T: Into<PathBuf>>(mut self, path: T, contents: &[u8]) -> Self {
        let path = path.into();
        let full_path = if path.is_relative() {
            self.path.join(path)
        } else {
            path
        };

        self.files.push(File::new(full_path, contents));

        self
    }
}

impl AsMut<Dir> for Dir {
    fn as_mut(&mut self) -> &mut Dir {
        self
    }
}

/// Represents a file stored in the filesystem. Contains the path and the contents in bytes.

#[derive(Clone, Debug)]
pub struct File {
    pub path: PathBuf,
    contents: Vec<u8>,
}

impl File {
    pub fn new<T: Into<PathBuf>>(path: T, contents: &[u8]) -> File {
        File {
            path: path.into(),
            contents: contents.into(),
        }
    }
}

/// Adds common path-based function. This allows a path-based type to create directories. mkdir_p
/// will recursively create a directory and all of its parent components if they are missing while
/// mkdir will create a single directory.

pub(crate) trait FilePath {
    fn mkdir_p(&self) -> Result<(), Box<dyn Error>>;
}

impl FilePath for PathBuf {
    fn mkdir_p(&self) -> Result<(), Box<dyn Error>> {
        create_dir_all(self).map_err(|err| err.into())
    }
}
