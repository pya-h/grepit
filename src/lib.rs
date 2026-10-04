pub mod config;
pub mod gmatches;
use walkdir::WalkDir;

use crate::gmatches::*;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

// TODO: if the path ends with `/` like folder/ it means that
// program needs to walk through all the files inside the path
pub fn run(cfg: config::Config) -> Result<(), Box<dyn Error>> {
    let filesnames = if cfg.filename().ends_with("/") {
        WalkDir::new(cfg.filename())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| entry.file_type().is_file())
            .map(|f| f.path().to_string_lossy().into_owned())
            .collect()
    } else {
        vec![cfg.filename()]
    };
    for filename in filesnames {
        let mut file = File::open(&filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        drop(file);
        let matches = GiMatches::findall(&cfg.query(), &contents, cfg.options());
        if matches.count() > 0 {
            println!(
                "File:{} Report\n- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - \n{:?}",
                filename,
                matches.report()
            );
            let mut lines: Vec<String> = contents.lines().map(|x| x.to_string()).collect();
            if cfg.options().replace_by.is_some() {
                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(filename)?;
                for item in matches.items {
                    if let Some(replacement) = item.replacement {
                        lines[item.line_number] = replacement;
                    }
                }
                file.write_all(lines.join("\n").as_bytes())?;
            }
        }
    }
    Ok(())
}
