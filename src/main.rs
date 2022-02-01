use std::env;
use std::process;

use minigrepv9::Config;

/// Search for a string in file
/// #Examples
///
/// ```
/// cargo run you poems.txt
/// ```
fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrepv9::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    // let contents =
    //     fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);
}
