use crate::commands::create_new_file;
use crate::types::{DIR_MODE, Inode};

pub fn handle_commands(commands: Vec<String>, actual_inode: Inode) -> Option<Inode> {
    match commands[0].as_str() {
        "help" => {
            let help_commands = commands[1..].to_vec();
            handle_help(help_commands);
            Some(actual_inode)
        }
        "new" => {
            let new_commands = commands[1..].to_vec();
            match handle_new(new_commands, actual_inode) {
                Ok(new_node) => Some(new_node),
                Err(error) => {
                    println!("{}", error);
                    None
                }
            }
        }
        "exit" => None,
        _ => {
            println!("Command not found. Type 'help' to see the list of available commands");
            Some(actual_inode)
        }
    }
}

fn handle_help(commands: Vec<String>) {
    if commands.is_empty() {
        println!(
            "Command options:
            help: show the list of available commands
            new <file|directory> <name>: create a new file or directory
            exit: exit the program"
        );
    } else if commands.len() > 1 {
        println!("Too many arguments for help command")
    } else {
        match commands[0].as_str() {
            "new" => println!("new <file|directory> <name>: create a new file or directory"),
            _ => println!("Command not found. Type 'help' to see the list of available commands"),
        }
    }
}

fn handle_new(commands: Vec<String>, hard_link: Inode) -> Result<Inode, &'static str> {
    if commands.len() != 2 {
        return Err("Invalid number of arguments, type 'help new' to see the usage of the command");
    }
    if commands[0] == "file" {
        if commands[1].is_empty() || commands[1].contains("/") || commands[1].contains("\\") {
            return Err("Invalid name for new file");
        }
        let new_file = create_new_file(commands[1].clone(), hard_link);
        Ok(new_file)
    } else if commands[0] == "directory" {
        if commands[1].is_empty() || commands[1].contains("/") || commands[1].contains("\\") {
            return Err("Invalid name for new directory");
        }
        let new_directory = Inode::new(DIR_MODE, commands[1].clone(), Some(Box::new(hard_link)));
        Ok(new_directory)
    } else {
        return Err("Invalid type of new content, type 'help new' to see the usage of the command");
    }
}
