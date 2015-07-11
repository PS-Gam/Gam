use std::io::{self,BufRead,BufReader};
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::net::{TcpStream, TcpListener};
use std::thread;
use std::convert::AsRef;

fn main() {
    fn get_challenge<'a>() -> (&'a str, &'a str) {
        ("Are you having fun? (y/n)", "y")
    }
    static count: AtomicUsize = ATOMIC_USIZE_INIT;
    let stdin = io::stdin();
    println!("Hello! Welcome to my game!");
    println!("Networked? (y/n)");
    let response = stdin.lock().lines().next().unwrap().unwrap();
    let networked = response == "y";

    if networked {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        
        println!("{}", listener.local_addr().unwrap());

        fn handle_client(stream: TcpStream) {
            loop {
                let reader = BufReader::new(&stream);
                let response = reader.lines().next().unwrap().unwrap();

                match response.as_ref() {
                    "HELO" => println!("Hi!"),
                    "FUN y" => println!("Good!"),
                    "FUN n" => println!(";_;"),
                    _ => println!("what?"),
                }
            }
        }

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move|| {
                        // connection succeeded
                        handle_client(stream)
                    });
                }
                Err(e) => { println!("{:?}", e); }
            }
        }
    } else {
        loop {
            let (challenge, verify) = get_challenge();

            println!("{}", challenge);

            let response = stdin.lock().lines().next().unwrap().unwrap();

            if response != verify {
                println!("WELL SCREW YOU");
                process::exit(1);
            } else {
                count.fetch_add(1, Ordering::SeqCst);
            }
            println!("You have {} points", count.load(Ordering::SeqCst));
        }
    }
}

