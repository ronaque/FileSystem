use std::io::{self, stdout, Write};

mod commands_handler;
mod types;
mod utils;

const welcome: &str = "Welcome to VFS";

fn create_root() -> types::Inode {
    let root = types::Inode::new(types::DIR_MODE, String::from("/"), None);
    root
}

fn main() {
    utils::clear_screen();
    println!("{welcome}");
    let root = create_root();
    let mut actual_inode = root;
    loop {
        actual_inode.print_inode_path();

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

    println!("Goodbye!");
}
