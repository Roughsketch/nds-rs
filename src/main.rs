extern crate byteorder;
#[macro_use] extern crate clap;
#[macro_use] extern crate failure;

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
            .conflicts_with_all(&["build", "files"]))
        .arg(Arg::with_name("build")
            .short("b")
            .long("build")
            .requires("output")
            .conflicts_with_all(&["extract", "files"]))
        .arg(Arg::with_name("files")
            .short("f")
            .long("files")
            .conflicts_with_all(&["extract", "build"]))
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .conflicts_with("files"))
        .get_matches();

    let input = matches.value_of("input").unwrap();
    let path = Path::new(input);

    if !path.exists() {
        eprintln!("Input '{}' does not exist.", input);
        return Ok(())
    }

    let rom = match nds::rom::Rom::new(path) {
        Ok(rom) => rom,
        Err(why) => {
            eprintln!("Error reading rom: {:?}", why);
            return Ok(());
        }
    };

    if matches.is_present("files") {
        
    }

    if matches.is_present("extract") {
        rom.extract(path)?;
    }

    if matches.is_present("build") {
        
    }

    Ok(())
}
