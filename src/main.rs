use std::io::{self,BufRead};
use std::process;

fn main() {
    let stdin = io::stdin();
    println!("Hello! Welcome to my game!");
    loop {
        println!("Are you having fun? (y/n)");
        let response = stdin.lock().lines().next().unwrap().unwrap();
        if response != "y" {
            println!("WELL SCREW YOU");
            process::exit(1);
        }
    }
}

