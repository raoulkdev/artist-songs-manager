// Imports
mod auth;
mod command_handler;
mod library;

fn main() {
    // Setup account
    let mut user = auth::register();

    // Handle commands
    command_handler::handler(&mut user);
}
