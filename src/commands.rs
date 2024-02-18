use std::fmt::Error;

pub fn handle_commands(commands: Vec<String>) -> bool {
    match commands[0].as_str() {
        "help" => {
            let help_commands = commands[1..].to_vec();
            handle_help(help_commands);
            true
        }
        "new" => {
            let new_commands = commands[1..].to_vec();
            match handle_new(new_commands) {
                Ok(_) => true,
                Err(error) => {
                    println!("{}", error);
                    true
                }
            }
        }
        "exit" => false,
        _ => {
            println!("Command not found. Type 'help' to see the list of available commands");
            true
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

fn handle_new(commands: Vec<String>) -> Result<bool, &'static str> {
    if commands.len() != 2 {
        return Err("Invalid number of arguments, type 'help new' to see the usage of the command");
    }
    if commands[0] == "file" {
        if commands[1].is_empty() || commands[1].contains("/") || commands[1].contains("\\") {
            return Err("Invalid name for new file");
        }
        println!("Creating a new file");
        Ok(true)
    } else if commands[0] == "directory" {
        if commands[1].is_empty() || commands[1].contains("/") || commands[1].contains("\\") {
            return Err("Invalid name for new directory");
        }
        println!("Creating a new directory");
        Ok(true)
    } else {
        return Err("Invalid type of new content, type 'help new' to see the usage of the command");
    }
}
