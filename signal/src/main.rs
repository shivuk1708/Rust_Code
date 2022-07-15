extern crate chan;
use std::{error::Error, thread};

use signal_hook::{iterator::Signals, SIGTERM};

fn main() -> Result<(), Box<Error>> {
    let signals = Signals::new(&[SIGTERM])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
        }
    });

    Ok(())
}