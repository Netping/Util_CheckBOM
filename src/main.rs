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

    /*
    let mut qnty_s = "12";
    if qnty_s.contains(".0") {
        if qnty_s.len() > 2 {
            qnty_s = &qnty_s[0..qnty_s.len() - 2];
        }
    }
    let qnty_r = qnty_s.parse::<usize>();
    if qnty_r.is_ok() {
        println!("{}", qnty_r.unwrap());
    }
    */

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
