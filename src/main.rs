use std::io::{self, read_to_string, stdin};

mod commands;

fn main() {
    print!("\x1B[2J\x1B[1;1H"); // x1B = Escape, [2J = clear screen, x1B[1;1H = Move cursor x/y
    println!("Welcome to VFS");

    let mut command: String = String::new();

    println!("Type a command");
    io::stdin()
        .read_line(&mut command)
        .expect("Error on reading line");

    let mut command_vector: Vec<_> = command
        .trim()
        .split_whitespace()
        .map(str::to_string)
        .collect();

    let mut command_result: bool = commands::handle_commands(command_vector);

    while command_result {
        println!("Type a command");
        command.clear();
        io::stdin()
            .read_line(&mut command)
            .expect("Error on reading line");

        let mut command_vector: Vec<_> = command
            .trim()
            .split_whitespace()
            .map(str::to_string)
            .collect();

        command_result = commands::handle_commands(command_vector);
    }
    println!("Goodbye!");
}
