use crate::{
    rust_tools::{Edition, Manifest},
    Dir, File, FilePath, Project,
};
use std::{error::Error, fmt::Debug, fs::File as FsFile, io::Write, path::PathBuf};

#[cfg(feature = "nightly")]
use std::convert::TryInto;

#[derive(Clone, Debug)]
pub struct GenericBuilder {
    path: PathBuf,
    project: Project,
}

impl GenericBuilder {
    pub fn new<T>(path: T) -> GenericBuilder
    where
        T: Into<PathBuf> + Clone,
    {
        GenericBuilder {
            path: path.clone().into(),
            project: Project::new(path),
        }
    }
}

pub trait Builder: Clone + Debug + Sized {
    fn add_dir(self, dir: Dir) -> Self {
        self.project().dirs.push(dir.to_owned());

        println!("{:?}", &self.project().dirs);
        self
    }

    fn build(self) -> Result<Project, Box<Error>> {
        println!("{:?}", self.project().dirs);
        for dir in self.project().dirs.iter() {
            dir.path.mkdir_p()?;
            for file in dir.files.iter() {
                FsFile::create(&file.path)?.write_all(&file.contents)?;
            }
        }

        Ok(self.project())
    }

    fn project(&self) -> Project;
}

impl Builder for GenericBuilder {
    fn project(&self) -> Project {
        self.project.clone()
    }
}

#[derive(Clone, Debug)]
pub struct RustBuilder {
    path: PathBuf,
    project: Project,
    manifest: Manifest,
}

impl RustBuilder {
    pub fn new<T>(path: T) -> RustBuilder
    where
        T: Into<PathBuf> + Clone,
    {
        RustBuilder {
            project: Project::new(path.clone()),
            path: path.into(),
            manifest: Manifest::default(),
        }
    }

    #[cfg(feature = "nightly")]
    pub fn add_cargo_toml(mut self, manifest: Manifest) -> Result<Self, Box<Error>> {
        self.manifest = manifest;
        let contents: Vec<u8> = self.clone().manifest.try_into()?;
        self.project.dirs[0]
            .files
            .push(File::new(self.path.join("Cargo.toml"), &contents));
        Ok(self)
    }

    #[cfg(not(feature = "nightly"))]
    pub fn add_cargo_toml(mut self, manifest: Manifest) -> Result<Self, Box<Error>> {
        self.manifest = manifest;
        let contents: Vec<u8> = Ok(toml::to_string(&self.manifest)?.into_bytes())
            .map_err(|e: toml::ser::Error| Box::new(e))?;
        self.project.dirs[0]
            .files
            .push(File::new(self.path.join("Cargo.toml"), &contents));
        Ok(self)
    }

    pub fn edition(mut self, edition: Edition) -> Self {
        self.manifest.package.edition = edition;

        self
    }
}

impl Builder for RustBuilder {
    fn build(self) -> Result<Project, Box<Error>> {
        for dir in self.project().dirs.iter() {
            dir.path.mkdir_p()?;
            for file in dir.files.iter() {
                FsFile::create(&file.path)?.write_all(&file.contents)?;
            }
        }
        Ok(self.project())
    }

    fn project(&self) -> Project {
        self.project.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust_tools::{Edition, Manifest};
    #[test]
    fn rust_builder_creates_rust_project() {
        let path = PathBuf::from("foo");
        let config = Manifest::try_from(
            "foo",
            "0.1.0",
            &["foo <foo@bar.com>"],
            Some(Edition::Edition2018),
            None,
        )
        .unwrap();
        let project = RustBuilder::new(&path)
            .add_cargo_toml(config)
            .unwrap()
            .build()
            .unwrap();

        assert!(path.exists());
        let path = path.join("Cargo.toml");
        assert!(path.exists());

        project.clear();
    }

    #[test]
    fn project_empty_build_creates_dir() {
        let path = PathBuf::from("tmp");
        let project = GenericBuilder::new(&path).build().unwrap();
        assert!(path.exists());
        project.clear();
    }

    #[test]
    fn project_with_dir_and_files_works() {
        let path = PathBuf::from("tmp2");
        let project = GenericBuilder::new(&path)
            .add_dir(Dir::new("tmp2/foo").add_file("bar", &vec![101u8]))
            .build().unwrap();

        assert!(path.exists());
        let path = path.join("foo");
        assert!(path.exists());
        let path = path.join("bar");
        assert!(path.exists());
//        project.clear();
    }

}


