pub mod header;
pub mod rom;

use failure::Error;
use std::path::Path;

#[fail(display = "Missing required directory: '{}'.", _0)]
#[derive(Clone, Debug, Fail)]
struct MissingFolderError(&'static str);

#[fail(display = "Missing required file: '{}'.", _0)]
#[derive(Clone, Debug, Fail)]
struct MissingFileError(&'static str);

pub fn valid_path<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let root = path.as_ref();

    //  Check root
    ensure!(root.is_dir(), MissingFolderError("root"));

    //  Check system directories
    ensure!(root.join("files").is_dir(), MissingFolderError("files"));
    ensure!(root.join("sys").is_dir(), MissingFolderError("sys"));
    ensure!(root.join("overlay").is_dir(), MissingFolderError("overlay"));

    //  Check system files
    ensure!(root.join("sys/arm9_overlay.bin").is_file(), MissingFileError("sys/arm9_overlay.bin"));
    ensure!(root.join("sys/arm7_overlay.bin").is_file(), MissingFileError("sys/arm7_overlay.bin"));
    ensure!(root.join("sys/arm9.bin").is_file(), MissingFileError("sys/arm9.bin"));
    ensure!(root.join("sys/arm7.bin").is_file(), MissingFileError("sys/arm7.bin"));
    ensure!(root.join("sys/fnt.bin").is_file(), MissingFileError("sys/fnt.bin"));
    ensure!(root.join("sys/fat.bin").is_file(), MissingFileError("sys/fat.bin"));
    ensure!(root.join("sys/header.bin").is_file(), MissingFileError("sys/header.bin"));

    Ok(())
}