use std::env;
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

    let results = search(&config.query, &contents, config.case_sensitive);

    println!();
    println!("Found {} results", results.len());
    for line in results {
        println!(": {}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    if case_sensitive {
        contents.lines().filter(|x| x.contains(query)).collect()
    } else {
        let query = &query.to_lowercase();
        contents.lines().filter(|x| x.to_lowercase().contains(query)).collect()
    }
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let mut case_sensitive = match env::var("CASE_INSENSITIVE") {
            Err(_) => false,
            Ok(case_ins_val) => case_ins_val == "yes",
        };

        for i in 3..args.len() {
            match args[i].as_str() {
                "--nocase" => case_sensitive = false,
                "--case" => case_sensitive = true,
                arg => return Err(leak_into_str(format!("Invalid argument {}", arg))),
            };
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive,
        })
    }
}

//Do not use lightly! Will never free string until program exits.
//Mostly necessary for formatted error messages, since they need
//a static lifetime anyways.
fn leak_into_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
