use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = parse_config(&args);

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
    fn new(args: &[String]) -> Config {
        Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        }
    }
}

fn parse_config(args: &[String]) -> Config {
    Config::new(args)
}
