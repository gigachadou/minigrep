#![allow(unused)]

use std::{env, error::Error, fs};
use std::process;

use minigrep::{search, search_case_insensitive};

fn main() {
    let ar_vec: Vec<String> = env::args().collect();
    
    let config = Configs::build(&ar_vec).unwrap_or_else(|err| {
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
    fn build(itr: &[String]) -> Result<Configs, &'static str> {
        if itr.len() < 3 {
            return Err("Not enough Arguments");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Configs{ query: itr[1].clone(), file_path: itr[2].clone(), ignore_case: ignore_case})
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
