use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if ["--help", "--h", "-help", "-h", "help", "h"].contains(&args[1].as_str()) {
        println!(
            "\
Usage: minigrep <query> <filename> args
Possible arguments:
--case: Does search using case sensitive mode
--nocase: Does search using case insensitive mode

You can also set case sensitivity mode using an environment variable:
CASE_INSENSITIVE will cause every search to be case insensitive by default if set to \"yes\" (unless overriden by an argument)
"
        );
        
        process::exit(0);
    }

    let config: Config = match Config::new(&args) {
        Ok(config) => config,
        Err(e) => {
            println!("Failed to parse config: \"{}\"", e);

            process::exit(1);
        }
    };

    if let Err(e) = minigrep::run(&config) {
        println!("Minigrep failed: \"{}\"", e);

        process::exit(2);
    }
}
