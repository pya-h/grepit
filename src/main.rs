extern crate minigrep;

use std::process;

use minigrep::config::Config;

fn main() {
    let cmd_args: Vec<String> = std::env::args().collect();
    let config = Config::new(&cmd_args).unwrap_or_else(|err: String| {
        println!("minigrep failed: {}", err);
        process::exit(1);
    });
    
    println!("{}", config.to_string());

    if let Err(err) = minigrep::run(config) {
        println!("Process failed: {}", err);
        process::exit(1);
    }
}
