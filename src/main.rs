mod editor;
use crate::editor::print_file;
use crate::editor::read_keys;
use std::fs;
use std::process;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() == 1 {
        read_keys();
    } else if args.len() == 2 {
        let input = &args[1];

        let mut contents = fs::read_to_string(&input).unwrap_or_else(|_err| {
            println!("Error reading file");
            process::exit(5);
        });
        println!("{}", contents);
        print_file(&mut contents);
    } else {
        println!("Too many arguments provided");
        exit(1);
    }
}
