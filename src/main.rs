mod scanner;
mod cursor;
mod token;

use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: lerlang [script]");
    }
    else {

        let filename = match args.get(1) {
            Some(x) => x,
            None => panic!("There was not a second argument found in the command line argument list. This is bad"),
        };

        // TODO: Find a more graceful way to hand this than to return a stupid dummy string that won't match on "ll"
        let extension = match Path::new(filename).extension() {
            Some(x) => x,
            None => OsStr::new("not_ler_lang"),
        };
        
        if extension.eq("ll") {
            run_file(filename);
        }
        else {
            println!("This is not a lerlang file")
        }
    }
}

fn run_file(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
}
