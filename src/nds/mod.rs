mod build;
mod extract;

pub use self::build::Builder;
pub use self::extract::Extractor;


pub fn valid_directory(path: String) -> Result<(), String> {
    if let Err(why) = Builder::is_nds_dir(path) {
        return Err(why.to_string())
    }

    Ok(())
}