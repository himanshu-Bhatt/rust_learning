use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    if config.case_sensitive {
        for line in search(&config.query, &contents) {
            println!("{}", line);
        }
    } else {
        for line in search_case_sensitive(&config.query, &contents) {
            println!("{}", line);
        }
    }

    Ok(())
}
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Less number of args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        return Ok(Config {
            query,
            filename,
            case_sensitive,
        });
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    return results;
}
pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let _query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&_query) {
            results.push(line)
        }
    }
    return results;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let content = "Rust productive";

        assert_eq!(vec!["Rust productive"], search(query, content));
    }

    #[test]
    fn search_case_insenitive() {
        let query = "Duct";
        let content = "Rust productive";

        assert_eq!(
            vec!["Rust productive"],
            search_case_sensitive(query, content)
        );
    }
}
