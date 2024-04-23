use crate::commands::create_new_file;
use crate::types::{DIR_MODE, Inode};

pub fn handle_commands(commands: Vec<String>, actual_inode: &mut Inode) -> bool {
    match commands[0].as_str() {
        "help" => {
            let help_commands = commands[1..].to_vec();
            handle_help(help_commands);
            false

        }
        "new" => {
            let new_commands = commands[1..].to_vec();
            match handle_new(new_commands, actual_inode) {
                Ok(()) => {
                    false
                },
                Err(error) => {
                    println!("{}", error);
                    false
                }
            }
        }
        "exit" => true,
        _ => {
            println!("Command not found. Type 'help' to see the list of available commands");
            false
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

fn handle_new(commands: Vec<String>, parent_inode: &mut Inode) -> Result<(), &'static str> {
    if commands.len() != 2 {
        return Err("Invalid number of arguments, type 'help new' to see the usage of the command");
    }
    if commands[0] == "file" {
        if commands[1].is_empty() || commands[1].contains("/") || commands[1].contains("\\") {
            return Err("Invalid name for new file");
        }
        match create_new_file(commands[1].clone(), parent_inode) {
            Ok(()) => {
                println!("Directory Meta-data: {:#?}", parent_inode);
                Ok(())
            },
            Err(error) => Err(error),
        }
    } else if commands[0] == "directory" {
        if commands[1].is_empty() || commands[1].contains("/") || commands[1].contains("\\") {
            return Err("Invalid name for new directory");
        }
        let new_directory = Inode::new(DIR_MODE, commands[1].clone());
        parent_inode.add_inode(new_directory);
        println!("Parent directory Meta-data: {:#?}", parent_inode);

        Ok(())
    } else {
        return Err("Invalid type of new content, type 'help new' to see the usage of the command");
    }
}
