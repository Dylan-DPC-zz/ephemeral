use std::{path::PathBuf, error::Error, io::Write, fmt::Debug};
use std::fs::{create_dir, create_dir_all, remove_dir_all, File as FsFile};

#[derive(Clone, Debug)]
pub struct Project {
    path: PathBuf,
    dirs: Vec<Dir>,
}

impl Project
{
    pub fn new<T>(path: T) -> Project
        where T: Into<PathBuf> + Clone
    {
        Project {
            path: path.clone().into(),
            dirs: vec![Dir::new(path)],
        }
    }

    pub fn build(self) -> Self {
        let f = self.dirs.get(0).unwrap().path.mkdir_p();
        self.dirs.iter().for_each(|dir| {
            dir.path.mkdir_p().expect("cannot create directory");

            dir.files.iter().for_each(|file| {
                let mut fs_file = FsFile::create(&file.path).expect("cannot create file");
                fs_file.write_all(&file.contents).expect("cannot write to the file");
            })
        });

        self
    }

    pub fn add_dir(mut self, directory: Dir) -> Self {
        self.dirs.push(directory);

        self
    }

    pub fn clear(self) {
       remove_dir_all(&self.dirs.get(0).unwrap().path).expect("can't delete path")
    }
}


#[derive(Clone, Debug)]
pub struct Dir {
   pub path: PathBuf,
   files: Vec<File>
}

impl Dir
{
    pub fn new<T: Into<PathBuf>>(path: T)-> Dir {
        Dir { path: path.into(), files: vec![]}
    }

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

impl AsMut<Dir> for Dir
{
    fn as_mut(&mut self) -> &mut Dir {
        self
    }
}

#[derive(Clone, Debug)]
pub struct File {
    pub path: PathBuf,
    contents: Vec<u8>
}

impl File
{
    pub fn new<T: Into<PathBuf>>(path: T, contents: &[u8]) -> File {
        File { path: path.into(), contents: contents.into() }
    }
}

trait FilePath {
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
