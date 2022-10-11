use std::env::args;
use std::process;

use minigrep::Config;


fn main() {

    let args: Vec<String> = args().collect();

    // |err| is called closures
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);

    });

    println!("query =  {}", config.query);
    println!("file_path = {}", config.file_path);

    println!("Starting to read contents of file_path={}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error = {}", e);
        process::exit(1);
    };

}
