use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config: Config = match Config::new(&args) {
        Ok(config) => config,
        Err(e) => panic!("Failed to parse config: {}", e),
    };

    run(&config)
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", &config.query);
    println!("In file {}", &config.filename);

    //Use of ? operator: if read to string returns an error,
    //will stop executing this function and return that error.
    let contents: String = fs::read_to_string(&config.filename)?;

    println!("With text:\n{}", contents);
    Ok(())
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
