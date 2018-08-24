use failure::Error;

use std::path::Path;

use nds::valid_path;
use nds::header::Header;

pub struct Rom {
    header: Header,
}

#[fail(display = "Invalid NDS rom or directory.")]
#[derive(Clone, Debug, Fail)]
pub struct InvalidRomError;

impl Rom {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let root = path.as_ref();

        let header = if root.is_file() {
            Header::new(root)?
        } else if valid_path(root).is_ok() {
            Header::new(root.join("header.bin"))?
        } else {
            bail!(InvalidRomError);
        };
        
        Ok(Self {
            header,
        })
    }

    pub fn extract<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        use std::fs::{File, create_dir_all};

        let root = path.as_ref();

        create_dir_all(root)?;
        create_dir_all(root.join("files"))?;
        create_dir_all(root.join("overlay"))?;
        create_dir_all(root.join("sys"))?;

        let mut header = File::open(root.join("header.bin"))?;
        self.header.write(&mut header)?;

        Ok(())
    }
}