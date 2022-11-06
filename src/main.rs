use std::env;
use std::process;
// library crate - to import something from lib crate
// use the projectname::what_you_want_to_import
use minigrep::run;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filepath);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
