pub mod config;
pub mod gmatches;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use crate::gmatches::*;

pub fn run(cfg: config::Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(cfg.filename())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File successfully read:\n\t{}\n", contents);
    let matches = GiMatches::findall(&cfg.query(), &contents, cfg.options());
    println!("{:?}", matches.report());
    Ok(())
}