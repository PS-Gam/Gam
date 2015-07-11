use std::io::{self,BufRead};
use std::process;

fn main() {
    fn get_challenge<'a>() -> (&'a str, &'a str) {
        ("Are you having fun? (y/n)", "y")
    }
    let mut count = 0;
    let stdin = io::stdin();
    println!("Hello! Welcome to my game!");
    loop {
        let (challenge, verify) = get_challenge();

        println!("{}", challenge);

        let response = stdin.lock().lines().next().unwrap().unwrap();
        if response != verify {
            println!("WELL SCREW YOU");
            process::exit(1);
        }
        else {
            count += 1;
        }
        println!("You have {} points", count);
    }
}

