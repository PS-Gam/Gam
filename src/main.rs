extern crate getopts;
extern crate rand;
use getopts::Options;
use std::env;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::io::{self,BufReader};
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::net::{TcpStream, TcpListener};
use std::thread;
use std::convert::AsRef;

fn get_challenge<'a>() -> (&'a str, &'a str) {
    println!("You have {} points", count.load(Ordering::SeqCst));
    let score = get_score();
    let rand_var = rand_int(score) % score;
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
fn rand_int(i : u64) -> u64 {
    let mut ans = i;
    for _ in 0..50 {
        // This seems legit ...
        ans = (ans * 17345 + 13989870) % 9223372036857
    }
    ans as u64
}

static threads: AtomicUsize = ATOMIC_USIZE_INIT;
static points_per_click:AtomicUsize = ATOMIC_USIZE_INIT;
static count: AtomicUsize = ATOMIC_USIZE_INIT;

fn handle_feed_network() {
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
}

fn main() {
    handle_cli_options();
    // println!("Hello! Welcome to my game!");
    // println!("Networked? (y/n)");
    // let stdin = io::stdin();
    // let response = stdin.lock().lines().next().unwrap().unwrap();

    // if response == "y" {
    //     handle_feed_network();
    // } else {
    //     handle_feed_stdin();
    // }
}

struct Upgrade<'a> {
    name: &'a str,
    desc: &'a str,
    cost: u64,
    activate: fn(),
    can_purchase: fn() -> bool,
}

fn sayhi() { println!("Hi") }

fn always() -> bool { true }

const upgrades : [Upgrade<'static>; 1] = [Upgrade {
    name: "Say Hi",
    desc: "Says hi to you on stdout",
    cost: 5,
    activate: sayhi,
    can_purchase: always,
}];

fn list_upgrades() {
    for u in upgrades.iter() {
        if (u.can_purchase)() {
            println!("Name: {}", u.name);
            println!("Cost: {}", u.cost);
            println!("Description: {}", u.desc);
        }
    }
}

fn buy_upgrade(upgrade : &Upgrade) {
    sub_score(upgrade.cost);
    (upgrade.activate)()
}

fn control_interface(stream: TcpStream) {
    loop {
        let reader = BufReader::new(&stream);
        let response = reader.lines().next().unwrap().unwrap();
        let args = response.split(" ");

        match response.as_ref() {
            "HELP" => println!("HELP, BUY, STATUS, LIST"),
            "LIST" => list_upgrades(),
            "STATUS" => {
                println!("Points: {}", count.load(Ordering::SeqCst))
            },
            "BUY y" => {
                buy_upgrade(&upgrades[0]);
                println!("Thank you for your purchase!")
            },
            _ => println!("what?"),
        }
    }
}

fn print_usage(opts: Options) {
    let program: String = env::args().next().unwrap();
    let brief = format!(
        "Usage: {0} help \n\
         Usage: {0} status \n\
         Usage: {0} upgrade list \n\
         Usage: {0} upgrade buy \n\
         Usage: {0} feed stdin \n\
         Usage: {0} feed net \
         ", program);
    print!("{}", opts.usage(&brief));
}

fn handle_cli_options() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }

    if matches.free.is_empty() {
        print_usage(opts);
        return;
    };

    match matches.free[0].as_ref() {
        "help" => print_usage(opts),
        "status" => println!("Points: {}", get_score()),
        "upgrade" => match matches.free[1].as_ref() {
            "list" => list_upgrades(),
            "buy" => {
                buy_upgrade(&upgrades[0]);
                println!("Thank you for your purchase!")
            },
            _ => print_usage(opts),
        },
        "feed" => match matches.free.get(1).map(|s| &**s) {
            Some("stdin") => handle_feed_stdin(),
            Some("net") => println!("do net feeding"),
            _ => print_usage(opts),
            },
        _ => print_usage(opts),
    }
}

fn handle_feed_stdin() {
    let stdin = io::stdin();
    let mut dest = fs::OpenOptions::new().write(true).create(true).append(true).open("score").unwrap();
    loop {
        let (challenge, verify) = get_challenge();

        println!("{}", challenge);

        let response = stdin.lock().lines().next().unwrap().unwrap();

        if response != verify {
            println!("WELL SCREW YOU");
            process::exit(1);
        } else {
            println!("RAndo num {}", rand_int(get_score()));
            dest.write_fmt(format_args!("{}", response));
            count.fetch_add(points_per_click.load(Ordering::SeqCst), Ordering::SeqCst);
        }
        println!("You have {} points", get_score());
        println!("You have {} points", count.load(Ordering::SeqCst));
    }
}

fn sub_score(amount : u64) {
    let mut dest = fs::OpenOptions::new().write(true).create(true).append(true).open("score").unwrap();

    let size = dest.metadata().unwrap().len();
    println!("Old score: {}", size);
    println!("cost: {}", amount);
    let newsize = size - amount;
    println!("New score: {}", newsize);

    dest.set_len(newsize);
    count.fetch_sub(amount as usize, Ordering::SeqCst);
}

fn get_score() -> u64 {
    let metadata = match fs::metadata("score") {
        Err(e) => {
            panic!("Error: {}", e);
        },
        Ok(f) => f,
    };

    metadata.len()
}
