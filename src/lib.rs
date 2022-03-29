use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, false));
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("<{}> Searching for {}", &config.filename, &config.query);

    //Use of ? operator: if read to string returns an error,
    //will stop executing this function and return that error.
    let contents: String = fs::read_to_string(&config.filename)?;

    //println!("With text:\n{}", contents);

    let results = search(&config.query, &contents, true);

    println!();
    println!("Found {} results", results.len());
    for line in results {
        println!(": {}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut results = Vec::new();

    if case_sensitive {
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
    } else {
        let query_lowercase = &query.to_lowercase();
        for line in contents.lines() {
            if line.to_lowercase().contains(query_lowercase) {
                results.push(line);
            }
        }
    }

    results
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
