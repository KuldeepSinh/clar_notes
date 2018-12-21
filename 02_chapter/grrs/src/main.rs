use signal_hook::{iterator::Signals, SIGINT};
use std::env;
use std::process;
use std::{error::Error, thread};

use grrs;
use grrs::Query;

fn main() -> Result<(), Box<Error>> {
    let signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            handle_interrupt(sig);
        }
    });

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

    Ok(())
}

fn handle_interrupt(sig: i32) {
    println!("Exiting with interrupt code {}.\n", sig);
    process::exit(-2);
}
