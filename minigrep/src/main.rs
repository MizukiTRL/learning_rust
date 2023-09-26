#![allow(unused)]
use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("error: {}", err);
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("in file {}", config.filename);

    if let Err(e) = run(&config){
        println!("application error: {e}");
        process::exit(1);
    }
}

