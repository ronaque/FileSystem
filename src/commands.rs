pub fn handle_commands(commands: Vec<String>) {
    match commands[0].as_str() {
        "help" => println!("Help command"),
        _ => todo!(),
    }
}
