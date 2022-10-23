use std::error::Error;
use std::fs;
use std::{env, process};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //same as the  line after this one
    // let contents = fs::read_to_string(config.file_path).expect("Error reading file");
    let contents = fs::read_to_string(config.file_path)?;

    let results = search(&config.query, &contents);

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub file_path: String,
    pub query: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Requires 3 arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { file_path, query })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
