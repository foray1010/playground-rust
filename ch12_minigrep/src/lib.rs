mod config;
mod search;
mod utils;

use std::error::Error;

pub use self::config::Config;
use self::search::*;
use self::utils::*;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = read_file_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    if !results.is_empty() {
        println!("{}", results.join("\n"));
    }

    Ok(())
}
