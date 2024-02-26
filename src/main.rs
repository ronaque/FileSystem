use std::io::{self, stdout, Write};
use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crossterm::terminal::{Clear, ClearType};

mod commands_handler;
mod types;
mod utils;
mod commands;

fn create_root() -> types::Inode {
    let root = types::Inode::new(types::DIR_MODE, String::from("/"), None);
    root
}

fn main() {
    let mut terminal = stdout();
    terminal.queue(Clear(ClearType::All)).unwrap();
    terminal.queue(MoveTo(0,0)).unwrap();
    terminal.write(b"Welcome to VFS\n").unwrap();
    let root = create_root();
    let mut actual_inode = root;
    loop {
        actual_inode.print_inode_path(&mut terminal);

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Error on reading line");

        let command_vector: Vec<String> = command
            .trim()
            .split_whitespace()
            .map(String::from)
            .collect();

        actual_inode = match commands_handler::handle_commands(command_vector, actual_inode) {
            Some(inode) => inode,
            None => break,
        };
    }
    println!("Goodbye! See you soon!");
}
