// Imports
use crate::auth::User;
use std::io::{Write, stdin, stdout};

// Command handler
pub fn handler(user: &mut User) {
    loop {
        // Command input values
        let mut command_input = String::new();
        let command;

        // Handle command input
        print!("artist_mngr -> ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut command_input)
            .expect("Could not read command input!");
        let command_input = command_input.trim().replace(" ", "");
        command = &command_input[0..command_input.len()];

        // Parse input and run command
        match command {
            "help" => help(),
            "who_is_artist" => who_is_artist(user),
            "new" => user.library.new_song(),
            "ls_songs" => user.library.list_all_songs(),
            _ => println!("Invalid command! Use 'help' to show all commands"),
        };
    }
}

// Display all commands
fn help() {
    println!("All commands are:");
    println!("  'who_is_artist' -> List the current user");
    println!("  'help' -> Show this command again");
    println!("  'ls_songs' -> List all songs ");
    println!("  'new' -> Create new song ");
}

// Display currently logged-in user
fn who_is_artist(user: &User) {
    println!("Current user: {}", user.username);
}
