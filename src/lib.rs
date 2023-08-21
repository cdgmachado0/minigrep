use std::error::Error;
use std::fs;
use std::env;
use std::str::ParseBoolError;


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config { 
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Self, Box<dyn Error>> { 
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err(Box::<dyn Error>::from("Didn't get a query string")),
        };

        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err(Box::<dyn Error>::from("Didn't get a file path")),
        };

        let (_, remaining) = args.size_hint();
        
        let ignore_case = if remaining.unwrap_or(0) == 1 {
            match args.next() {
                Some(is_case) => {
                    let case: Result<bool, ParseBoolError> = is_case.parse();
                    match case {
                        Ok(case) => case,
                        Err(e) => {
                            let err_msg = format!("Invalid case flag: {}", e);
                            return Err(Box::<dyn Error>::from(err_msg));
                        }
                    }
                },
                None => return Err(Box::<dyn Error>::from("Didn't get a case option")),
            }
        } else {
            env::var("IGNORE_CASE").is_ok()
        };

        Ok(Config { query, file_path, ignore_case })
    }
}


pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else { 
        search(&config.query, &contents)
    };

    for line in &results {
        println!("{line}");
    }

    Ok(results.iter().map(|el| el.to_string()).collect())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
Pick three.
Duck tape.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn cli_case_precedence_false() {
        env::set_var("IGNORE_CASE", "1");
        let args = vec![" ", "to", "poem.txt", "false"];
        let args: Vec<String> = args.iter().map(|el| el.to_string()).collect();

        let config = Config::build(&args).unwrap();
        let results = run(config).unwrap();

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn cli_case_precedence_true() {
        env::set_var("IGNORE_CASE", "1");
        let args = vec![" ", "to", "poem.txt", "true"];
        let args: Vec<String> = args.iter().map(|el| el.to_string()).collect();

        let config = Config::build(&args).unwrap();
        let results = run(config).unwrap();

        assert_eq!(results.len(), 4);
    }
}

