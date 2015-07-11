extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Edge, Pin};
use std::env;
use std::io::prelude::*;
use std::io::{stdout,Result};
use std::thread;

fn interrupt(pin : u64, outpin:u64) -> Result<()> {
    let output = Pin::new(outpin);
    output.export();
    output.set_direction(Direction::Out);
    let input = Pin::new(pin);
    input.export();
    input.set_direction(Direction::In);
    input.set_edge(Edge::BothEdges);
    let mut poller = try!(input.get_poller());
    loop {
        match try!(poller.poll(1000)) {
            Some(value) => {
                println!("{}", value);
                output.set_value(1-value);
            },
            None => {
                let mut stdout = stdout();
                try!(stdout.write(b"."));
                try!(stdout.flush());
            },
        }
    }
}

fn main() {
    let child1 = thread::spawn(move || {
        interrupt(17, 4);
    });
    let child2 = thread::spawn(move || {
        interrupt(16, 5);
    });
    child1.join();
    child2.join();
}
