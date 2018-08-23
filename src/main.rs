#[macro_use] extern crate clap;
#[macro_use] extern crate failure;

use clap::{Arg, App};
use failure::Error;

use std::fs::{File, create_dir_all};
use std::path::Path;

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

    if let Some(rom) = matches.value_of("files") {
        if !Path::new(rom).exists() {
            eprintln!("File '{}' does not exist.", rom);
            return Ok(())
        }
    }

    if let Some(rom) = matches.value_of("extract") {
        if !Path::new(rom).exists() {
            eprintln!("File '{}' does not exist.", rom);
        }
    }

    Ok(())
}
