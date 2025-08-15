mod scanner;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <host>", args[0]);
        process::exit(1);
    }

    let host = &args[1];

    println!("Start scanning at {}", host);

    if let Err(e) = scanner::scan_host(host) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
