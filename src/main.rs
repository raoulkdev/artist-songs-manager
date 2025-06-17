// Imports
mod auth;
mod command_handler;
mod library;

fn main() {
    let mut user = auth::register();
    command_handler::handler(&mut user);
}
