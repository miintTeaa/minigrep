use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = match parse_config(&args) {
        Ok(config) => config,
        Err(e) => panic!("Failed to parse config: {}", e),
    };

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Couldn't read file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    Config::new(args)
}
