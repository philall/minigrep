use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;


#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub query: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_SENSIT IVE").is_err(); 

        Ok(Config { query, filename, case_sensitive } )
    }
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config_args_bad() {
        let args  = vec![String::from("main"), String::from("foo")];
        let config = Config::new(&args);
        assert_eq!(config.is_err(), true);
        assert_eq!(config.unwrap_err(), "Not enough arguments");
    }

    #[test]
    fn test_config_args_good() {
        let args  = vec![String::from("main"), String::from("foo"), String::from("bla")];
        let config = Config::new(&args);
        assert_eq!(config.is_ok(), true);
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
fast, safe, productive
pick three.
Duck tape.";
        assert_eq!(
            vec!["fast, safe, productive"], 
            search(query, contents)
        );
        
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick tree.
Trust Me.";

        assert_eq!(
            vec!["Rust:", "Trust Me."],
            search_case_insensitive(query, contents)
        );
    }
}
