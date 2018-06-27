mod config;
mod search;
mod utils;

use std::error::Error;

pub use config::Config;
use search::*;
use utils::*;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let contents = read_file_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    if results.len() > 0 {
        println!("{}", results.join("\n"));
    }

    Ok(())
}
