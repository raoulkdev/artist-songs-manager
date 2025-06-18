// Imports
use crate::library::Library;
use rpassword::read_password;
use std::io::{Write, stdin, stdout};

// User struct
pub struct User {
    pub username: String,
    pub password: String,
    pub library: Library,
}

impl User {
    fn new(username: &str, password: &str) -> Self {
        Self {
            username: String::from(username),
            password: String::from(password),
            library: Library { songs: vec![] },
        }
    }
}

// Create user account
pub fn register() -> User {
    // New account credentials
    let mut new_username = String::new();
    let mut new_password = String::new();

    // New username input
    print!("New username: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut new_username)
        .expect("Could not read new username input!");
    let new_username = new_username.trim();

    // New password input
    print!("New password (all spaces are removed): ");
    stdout().flush().unwrap();
    new_password = read_password()
        .expect("Could not read password input")
        .replace(" ", "");
    let new_password = new_password.trim();

    // Return registered user
    User::new(new_username, new_password)
}
