use std::io::{self,BufRead};
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

fn main() {
    fn get_challenge<'a>() -> (&'a str, &'a str) {
        ("Are you having fun? (y/n)", "y")
    }
    static count: AtomicUsize = ATOMIC_USIZE_INIT;
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
            count.fetch_add(1, Ordering::SeqCst) ;
        }
        println!("You have {} points", count.load(Ordering::SeqCst));
    }
}

