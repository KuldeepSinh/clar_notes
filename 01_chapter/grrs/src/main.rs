use std::env;
use std::process;

use grrs;
use grrs::Query;

fn main() {
    let config = Query::new(env::args()).unwrap_or_else(|err| {
        //eprintln prints to stderr
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    if let Err(e) = grrs::run(config) {
        //eprintln prints to stderr
        eprintln!("Application error {}.", e);
        process::exit(1);
    }
}
