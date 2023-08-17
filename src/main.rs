use std::env;
use std::fs;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let contents = run(config).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    println!("With text:\n{contents}");
}

fn run(config: Config) -> Result<String, &'static str> {
    match fs::read_to_string(config.file_path) {
        Ok(string) => Ok(string),
        Err(_) => Err("Should have been able to read the file"),
    }
}



struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        } 

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}