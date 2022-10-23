use std::error::Error;
use std::fs;
use std::{env, process};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //same as the  line after this one
    // let contents = fs::read_to_string(config.file_path).expect("Error reading file");
    let mut contents = fs::read_to_string(config.file_path)?;

    {
        let results = if config.ignore_case {
            search_case_insensitive(&config.query, &mut contents)
        } else {
            search(&config.query, &mut contents)
        };
        // contents.clear(); Does not compile according to the life times defined in search
        for line in results {
            println!("{line}");
        }
    }

    Ok(())
}

pub struct Config {
    pub file_path: String,
    pub query: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            file_path,
            query,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //faster than using loops
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()

    //same as
    // let mut results: Vec<&str> = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //faster than using loops
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()

    //same as
    // let mut results: Vec<&str> = Vec::new();
    // let query = query.to_lowercase();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape
";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";

        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(query, contents)
        )
    }
}
