extern crate byteorder;
#[macro_use] extern crate clap;
#[macro_use] extern crate failure;
extern crate memmap;
extern crate num;

use clap::{Arg, App};
use failure::Error;

use std::path::Path;

mod nds;

fn main() -> Result<(), Error> {
    let matches = App::new("NDS Tool")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(Arg::with_name("extract")
            .short("e")
            .long("extract")
            .requires("output")
            .conflicts_with_all(&["build", "files"])
            .takes_value(true))
        .arg(Arg::with_name("build")
            .short("b")
            .long("build")
            .requires("output")
            .conflicts_with_all(&["extract", "files"])
            .takes_value(true)
            .validator(nds::valid_directory))
        .arg(Arg::with_name("files")
            .short("f")
            .long("files")
            .conflicts_with_all(&["extract", "build"])
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .conflicts_with("files"))
        .get_matches();

    if matches.is_present("files") {
        
    }

    if let Some(input) = matches.value_of("extract") {
        let path = Path::new(input);

        //  Check if input path is a file, since it must be a ROM.
        //  This also checks if it exists.
        if !path.is_file() {
            eprintln!("Input '{}' is not a valid file.", input);
            return Ok(())
        }

        let output = matches.value_of("output").unwrap();

        let rom = match nds::Extractor::new(path) {
            Ok(rom) => rom,
            Err(why) => {
                eprintln!("Error extracting rom: {:?}", why);
                return Ok(());
            }
        };

        rom.extract(output)?;
    }

    if let Some(input) = matches.value_of("build") {
        let path = Path::new(input);

        //  Check if input path is a directory, since it must be one.
        //  This also checks if it exists.
        if !path.is_dir() {
            eprintln!("Input '{}' is not a valid directory.", input);
            return Ok(())
        }

        let output = matches.value_of("output").unwrap();

        let rom = match nds::Builder::new(path) {
            Ok(rom) => rom,
            Err(why) => {
                eprintln!("Error building rom: {:?}", why);
                return Ok(());
            }
        };

        rom.build(output)?;
    }

    Ok(())
}
