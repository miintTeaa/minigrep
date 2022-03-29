use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", &config.query);
    println!("In file {}", &config.filename);

    //Use of ? operator: if read to string returns an error,
    //will stop executing this function and return that error.
    let contents: String = fs::read_to_string(&config.filename)?;

    println!("With text:\n{}", contents);
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}
