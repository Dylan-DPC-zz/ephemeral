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
//! ephemeral = "0.1"
//! ```
//!
//! # USAGE:
//!
//! To create a project:
//!
//! ```rust
//! use ephemeral::{Project, Dir};
//!
//! fn main() {
//!     let project = Project::new("tmp")
//!        .add_dir(Dir::new("tmp/foo").add_file("bar", &vec![101u8]))
//!        .build();
//!
//!     project.clear();
//! }
//! ```
//!
//! This will create a new project in a dir called `tmp` which will contain a dir "foo" which will
//! contain a file `bar` with `e` (101u8) written to the file.

use std::fs::{create_dir, create_dir_all, remove_dir_all, File as FsFile};
use std::{error::Error, io::Write, path::PathBuf};

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

    /// Creates the project in the filesystem. This will create all the directories & files that are
    /// added by using `add_dir()`.
    ///
    /// No function should be chained for this, except for `clear()`.
    ///
    /// Function panics if the directory or file cannot be created or written to.

    pub fn build(self) -> Self {
        self.dirs.iter().for_each(|dir| {
            dir.path.mkdir_p().expect("cannot create directory");

            dir.files.iter().for_each(|file| {
                let mut fs_file = FsFile::create(&file.path).expect("cannot create file");
                fs_file
                    .write_all(&file.contents)
                    .expect("cannot write to the file");
            })
        });

        self
    }

    /// Adds a directory to the chain which will be created when `build()` is called. This accepts
    /// a Dir, with the files already attached to it. To add a subdirectory, specify the path from
    /// the project root.

    pub fn add_dir(mut self, directory: Dir) -> Self {
        self.dirs.push(directory);

        self
    }

    /// Deletes the project from the filesystem. This function can be used to clear the project
    /// after running the tests.
    ///
    /// This function panics if a directory cannot be deleted.

    pub fn clear(self) {
        println!("{:?}", &self.dirs[0]);
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

pub trait FilePath {
    fn mkdir_p(&self) -> Result<(), Box<dyn Error>>;
    fn mkdir(&self) -> Result<(), Box<dyn Error>>;
}

impl FilePath for PathBuf {
    fn mkdir_p(&self) -> Result<(), Box<dyn Error>> {
        create_dir_all(self).map_err(|err| err.into())
    }

    fn mkdir(&self) -> Result<(), Box<dyn Error>> {
        create_dir(self).map_err(|err| err.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn project_empty_build_creates_dir() {
        let path = PathBuf::from("tmp");
        let project = Project::new(&path);
        project.clone().build();
        assert!(path.exists());
        project.clear();
    }

    #[test]
    fn project_with_dir_and_files_works() {
        let path = PathBuf::from("/home/dpc/Code/ephemeral/tmp2");
        let project = Project::new(&path)
            .add_dir(Dir::new("tmp2/foo").add_file("bar", &vec![101u8]))
                .build();

        assert!(path.exists());
        let path = path.join("foo");
        assert!(path.exists());
        let path = path.join("bar");
        assert!(path.exists());
        project.clear();
    }

}
