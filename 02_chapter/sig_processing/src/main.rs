extern crate signal_hook;

use signal_hook::{iterator::Signals, SIGINT};
use std::io;
use std::process;
use std::{error::Error, thread};

fn main() -> Result<(), Box<Error>> {
    let signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            handle_interrupt(sig);
        }
    });
    //get string (input from the user) from the command line
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read.");
    Ok(())
}

fn handle_interrupt(sig: i32) {
    println!("Exiting with interrupt code {}.\n", sig);
    process::exit(-2);
}
