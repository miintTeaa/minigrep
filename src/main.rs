use minigrep::{self, Config};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = match Config::new(&args) {
        Ok(config) => config,
        Err(e) => panic!("Failed to parse config: {}", e),
    };

    if let Err(e) = minigrep::run(&config) {
        panic!("Failed with error \"{}\"", e);
    }
}
