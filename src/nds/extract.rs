use byteorder::{LittleEndian, ReadBytesExt};
use failure::Error;
use memmap::Mmap;
use num::{Num, NumCast};

use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

pub struct Extractor {
    data: Mmap,
}

#[fail(display = "Invalid NDS rom or directory.")]
#[derive(Clone, Debug, Fail)]
pub struct InvalidRomError;

impl Extractor {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let root = path.as_ref();

        let file = File::open(root)?;
        
        Ok(Self {
            data: unsafe { Mmap::map(&file)? },
        })
    }

    pub fn extract<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let root = path.as_ref();

        create_dir_all(root)?;

        self.write(root.join("header.bin"), 0, 0x200)?;
        self.write(root.join("arm9.bin"), self.read_u32(0x20)?, self.read_u32(0x2C)?)?;
        self.write(root.join("arm7.bin"), self.read_u32(0x30)?, self.read_u32(0x3C)?)?;

        self.extract_overlays(root.join("overlay"))?;
        self.extract_files(root.join("data"))?;

        Ok(())
    }

    fn write<P, R1, R2>(&self, path: P, start: R1, end: R2) -> Result<(), Error>
        where
            P: AsRef<Path>,
            R1: Num + NumCast,
            R2: Num + NumCast
    {
        let start: usize = NumCast::from(start).unwrap();
        let end: usize = NumCast::from(end).unwrap();

        let mut file = File::create(path)?;
        file.write(&self.data[start..start + end])?;

        Ok(())
    }

    fn read_u32(&self, offset: usize) -> Result<u32, Error> {
        let value = (&self.data[offset..]).read_u32::<LittleEndian>()?;
        Ok(value)
    }
    
    fn extract_overlays<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        create_dir_all(path)?;

        Ok(())
    }
    
    fn extract_files<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        create_dir_all(path)?;

        Ok(())
    }
}