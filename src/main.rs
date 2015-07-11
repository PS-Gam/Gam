use std::io::{self,BufRead,BufReader};
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::net::{TcpStream, TcpListener};
use std::thread;
use std::convert::AsRef;

/*
Table of Constants
*/


fn main() {
    fn get_challenge<'a>() -> (&'a str, &'a str) {
        println!("You have {} points", count.load(Ordering::SeqCst));
        let rand_var = rand_int(count.load(Ordering::SeqCst) as i64) % count.load(Ordering::SeqCst) as i64;
        if rand_var < 5 {
            ("Are you having fun? (y/n)", "y")
        } else if rand_var < 10 {
            ("Are you a robot? Robots are not permitted to play Proof of Fun", "n")
        } else if rand_var < 20 {
            ("Please rate the game from 1 to 5", "5")
        } else if rand_var < 40 {
            ("Please rate the game from one to ten", "ten")
        } else if rand_var < 80 {
            ("Please rate the game from 1 to 5", "5")
            //let val = rand_int(rand_var);
            //(format!("If you're having fun, you can say this number back to me: {}", val), format!("{}", val))
        } else if rand_var < 160 {
            ("Please rate the game from 1 to 5", "5")
            //let num_char = rand_int(rand_var) % 15;
            //let answer = "";
            //let alphabet = "qwertasdfgzxcvbyuiophjklnm";
            //for i in 0..num_char {
            //    answer += alphabet.char_at(rand_int(i) as i64 % 26);
            //}
            //(format!("To express your fun, enter this pass phrase: {}", answer), format!("{}", answer))
        } else if rand_var < 320 {
            ("Please rate the game from 1 to 5", "5")
            //This is where we want to force the user to use adifferent port. NOT unlock more than one. 
            // Just that stdin doesn't work anymore. This is meant to be a pain in the ass, so they pay the 250? or so 
            // to buy another thread 
        }
        else {
            ("Please rate the game from 1 to 5", "5")
        }


    }
    // Random is hard to import ...
    fn rand_int(i : i64) -> i64 {
        let mut x = 1;
        let mut ans = i;
        for x in 0..50 {
            // This seems legit ...
            ans = (ans * 17345 + 13989870) % 9223372036857
        }
        ans as i64
    }

    static count: AtomicUsize = ATOMIC_USIZE_INIT;
    static threads: AtomicUsize = ATOMIC_USIZE_INIT;
    static points_per_click:AtomicUsize = ATOMIC_USIZE_INIT;
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
                println!("RAndo num {}", rand_int(count.load(Ordering::SeqCst) as i64));
                count.fetch_add(points_per_click.load(Ordering::SeqCst), Ordering::SeqCst);
            }
            println!("You have {} points", count.load(Ordering::SeqCst));
        }
    }
}

