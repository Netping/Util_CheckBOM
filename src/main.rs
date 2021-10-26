use checkbom::mg_mod::*;
use std::env;
//use std::error::Error;
//use std::fs;
use std::process;

fn main() {
    //let args: Vec<String> = env::args().collect();
    //let config = Config::new(&args).unwrap_or_else(|err| {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!(" File BOM1: {}", config.bom1);
    println!(" File BOM2: {}", config.bom2);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
