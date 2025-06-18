// Imports
mod auth;
mod command_handler;
mod library;
mod song;

fn main() {
    // Setup account
    let mut user = auth::register();

    // Handle commands
    command_handler::handler(&mut user);
}
