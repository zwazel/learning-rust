// Input/Output library, std is the standard library
use std::io;

use crate::numberguesser::numberguesser::guess_the_number;

mod numberguesser;

fn main() {
    println!("Hello, world!");
    loop {
        println!("- Type 0 to exit the game");
        println!("- Type 1 to play a number guessing game!");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command: u32 = match command.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match command {
            0 => { break; }
            1 => { guess_the_number() }
            _ => {
                println!("Unknown, pls type one of the specified numbers!");
                continue;
            }
        }
    }
}