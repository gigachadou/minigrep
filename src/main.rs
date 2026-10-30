#![allow(unused)]

use std::{env, error::Error, fs};
use std::process;

use minigrep::{search, search_case_insensitive};

fn main() {
    let config = Configs::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error while parsing: {err}");
        process::exit(1);
    });

    println!("Arguments: {}, {}", config.query, config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Error while parsing: {e}");
        process::exit(1);
    }
}

struct Configs{
    query: String,
    file_path: String,
    ignore_case: bool
}

impl Configs {
    fn build(mut args_i: impl Iterator<Item = String>) -> Result<Configs, &'static str> {
        args_i.next();

        let query = match args_i.next() {
            Some(arg) => arg,
            None => return  Err("Did not get the query")
        };

        let file_path = match args_i.next() {
            Some(arg) => arg,
            None => return Err("Did not get the path to search")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Configs{ query, file_path, ignore_case})
    }
}

fn run(config: Configs) -> Result<(),Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    //println!("In the File:\n{contents}");
    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &content) {
            println!("{line}")
        }
    } else {
        for line in search(&config.query, &content) {
            println!("{line}");
        }
    }

    Ok(())
}
