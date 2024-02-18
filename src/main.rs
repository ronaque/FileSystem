use std::io::{self, stdout, Write};

use crate::utils::clear_screen;

mod commands;
mod types;
mod utils;

const welcome: &str = "Welcome to VFS";

fn main() {
    clear_screen();
    println!("{welcome}");

    loop {
        print!("tmp/ > ");
        stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Error on reading line");

        let command_vector: Vec<String> = command
            .trim()
            .split_whitespace()
            .map(String::from)
            .collect();

        if !commands::handle_commands(command_vector) {
            break;
        }
    }

    println!("Goodbye!");
}
