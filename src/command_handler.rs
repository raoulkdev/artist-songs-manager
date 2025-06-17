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
        let command_input = command_input.to_lowercase();
        command = &command_input[0..command_input.len()];

        // Parse input and run command
        match command {
            "help" => help(),
            "exit" => {
                println!("Deleting account...");
                std::process::exit(0);
            }
            "who_is_user" => who_is_user(user),
            "new" => user.library.new_song(),
            "upt" => user.library.update_song(),
            "ls_songs" => user.library.list_all_songs(),
            "search_t" => {
                user.library.search_by_title();
            }
            "search_a" => {
                user.library.search_by_artists();
            }
            _ => println!("Invalid command! Use 'help' to show all commands"),
        };
    }
}

// Display all commands
fn help() {
    println!("All commands are:");
    println!("  'who_is_user' -> List the current user");
    println!("  'help' -> Show this command again");
    println!("  'ls_songs' -> List all songs ");
    println!("  'new' -> Create new song ");
    println!("  'upt' -> Update an existing song ");
    println!("  'search_t' -> Search song(s) by title ");
    println!("  'search_a' -> Search song(s) by artist ");
    println!("  'exit' -> Close this program ");
}

// Display currently logged-in user
fn who_is_user(user: &User) {
    let mut asterisks = String::new();
    
    for char in 0..user.password.chars().count() -3 {
        asterisks += "*";
    }
    
    println!("Current user: {}", user.username);
    println!("Password length: {}{}", String::from(&user.password[0..3]), asterisks);
}
