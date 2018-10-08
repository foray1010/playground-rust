extern crate ch12_minigrep;

use std::env;
use std::process;

use ch12_minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = ch12_minigrep::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
