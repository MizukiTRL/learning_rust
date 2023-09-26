#![allow(unused)]
use std::error::Error;
use std::{fs, env};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.filename)?;

    for line in search(&config.query, &contents){
        println!("{}", line);
    }

    Ok(())
}

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
        
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>{

        args.next();
        let query = args.next().expect("failed to read the argument");
        let filename = args.next().expect("failed to read the file name");
        
        return Ok(Config { query: query, filename: filename, case_sensitive: false });
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|s| s.contains(query))
        .collect()
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn one_result(){
        let query = "oy";
        let contents = "\
Hola perro,
soy gato
doc
        ";
        assert_eq!(vec!["soy gato"], search(query, contents));
    }
}

