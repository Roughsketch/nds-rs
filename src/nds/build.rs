use failure::Error;
use std::path::{Path, PathBuf};

#[fail(display = "Missing required directory: '{}'.", _0)]
#[derive(Clone, Debug, Fail)]
struct MissingFolderError(&'static str);

#[fail(display = "Missing required file: '{}'.", _0)]
#[derive(Clone, Debug, Fail)]
struct MissingFileError(&'static str);

pub struct Builder {
    root: PathBuf,
}

impl Builder {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let root = path.as_ref();
    
        Ok(Self {
            root: root.to_path_buf(),
        })
    }

    pub fn is_nds_dir<P: AsRef<Path>>(path: P) -> Result<(), Error> {
        let root = path.as_ref();

        //  Check root
        ensure!(root.is_dir(), MissingFolderError("root"));

        //  Check system directories
        ensure!(root.join("data").is_dir(), MissingFolderError("data"));
        ensure!(root.join("overlay").is_dir(), MissingFolderError("overlay"));

        //  Check system files
        ensure!(root.join("arm9_overlay.bin").is_file(), MissingFileError("arm9_overlay.bin"));
        ensure!(root.join("arm7_overlay.bin").is_file(), MissingFileError("arm7_overlay.bin"));
        ensure!(root.join("arm9.bin").is_file(), MissingFileError("arm9.bin"));
        ensure!(root.join("arm7.bin").is_file(), MissingFileError("arm7.bin"));
        ensure!(root.join("header.bin").is_file(), MissingFileError("header.bin"));

        Ok(())
    }

    pub fn build<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let output = path.as_ref();


        Ok(())
    }
}

