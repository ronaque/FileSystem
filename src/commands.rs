pub fn handle_commands(commands: Vec<String>) -> bool {
    match commands[0].as_str() {
        "help" => {
            println!(
                "Command options:
                help: show the list of available commands
                exit: exit the program"
            );
            return true;
        }
        "exit" => return false,
        _ => {
            println!("Command not found. Type 'help' to see the list of available commands");
            true
        }
    }
}
