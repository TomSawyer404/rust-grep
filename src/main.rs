//! # Note
//! This is a 'grep' program based on Rust.
//!
//! Usage:
//!
//! ```bash
//! rust-grep [pattern] [files...]
//! cat somefile.txt | rust-grep [pattern]
//! ```
//!
//! @auhtor:    Mrbanana
//! @date:      2021-10-4
//! @license:   The MIT License
//!
//! # Knowledge you will learn
//! - How to read a file in Rust
//! - How to get argv[] from enviroment variable
//! - How to search pattern from a string in Rust

use std::env;
use std::fs::*;
use std::io::*;

fn main() {
    // Get argv[] from env
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        eprintln!("Usage1: rust-grep [pattern] [files...]");
        eprintln!("Usage2: cat somefile.txt | rust-grep [pattern]");
        std::process::exit(1);
    }

    // Read file from stdin
    let mut buffer = [0u8; 4096];
    let pattern = argv[1].as_str();
    if argv.len() == 2 {
        loop {
            let read_bytes = std::io::stdin()
                .read(&mut buffer)
                .expect("Cannot read from stdin");
            filte_pattern(&buffer, pattern);

            if read_bytes < buffer.len() {
                break;
            }
        }
    } else {
        // Read file from argv
        for i in 2..argv.len() {
            let mut fd = File::open(argv[i].as_str()).expect("Cannot open file!");
            loop {
                let read_bytes = fd.read(&mut buffer).expect("Cannot read file");
                filte_pattern(&buffer, argv[1].as_str());

                if read_bytes < buffer.len() {
                    break;
                }
            }
        }
    }
}

fn filte_pattern(buffer: &[u8], pattern: &str) {
    for i in buffer
        .lines()
        .filter(|x| x.as_ref().unwrap().contains(pattern))
    {
        let replaced_str = format!("{}{}{}", "\x1b[31m", pattern, "\x1b[0m");
        let i = i.unwrap().replace(pattern, replaced_str.as_str());
        println!("{}", i);
    }
}
