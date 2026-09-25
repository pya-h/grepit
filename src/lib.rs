pub mod config;
pub mod gmatches;
use crate::gmatches::*;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

pub fn run(cfg: config::Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(cfg.filename())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    drop(file);
    let matches = GiMatches::findall(&cfg.query(), &contents, cfg.options());
    println!("{:?}", matches.report());
    let mut lines: Vec<String> = contents.lines().map(|x| x.to_string()).collect();
    if cfg.options().replace_by.is_some() {
        let mut file = OpenOptions::new().write(true).truncate(true).open(cfg.filename())?;
        for item in matches.items {
            if let Some(replacement) = item.replacement {
                lines[item.line_number] = replacement;
            }
        }
        file.write_all(lines.join("\n").as_bytes())?;
    }
    Ok(())
}
