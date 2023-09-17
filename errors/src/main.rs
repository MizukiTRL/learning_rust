#![allow(unused)]

use std::fs::File;
use std::io::ErrorKind;
use std::io::Error;
use std::io::Read;
use std::io;


fn main() {
    //panic!("owo")
    
    let f = File::open("hello.txt");

    match f{
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("text.txt") {
                Ok(nf) => nf,
                Err(error) => panic!("{:?}", error),
            },
            other_error => panic!("error found: {:?}", other_error),
        },
    };
}

fn read_from_file() -> Result<String, io::Error>{
    let mut f = File::open("hello.txt")?;

    /* 
    let mut f: File = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };*/

    let mut s = String::new();

    /* 
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }*/

    f.read_to_string(&mut s)?;
    Ok(s)
}
