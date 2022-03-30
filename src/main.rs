use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
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
