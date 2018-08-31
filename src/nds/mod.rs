mod build;
mod extract;
pub mod fs;

pub use self::build::Builder;
pub use self::extract::Extractor;

/// Validator for clap. Used to limit build command to directories
/// that are valid. 
/// 
/// Due to race conditions, the validity is not a guarantee that 
/// the directory is valid through the duration of program execution, 
/// so errors can still be thrown for missing files.
pub fn valid_directory(path: String) -> Result<(), String> {
    if let Err(why) = Builder::is_nds_dir(path) {
        return Err(why.to_string())
    }

    Ok(())
}