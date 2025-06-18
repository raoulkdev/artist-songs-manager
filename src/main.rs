// Imports
#[path = "user/auth.rs"]
mod auth;
#[path = "user/command_handler.rs"]
mod command_handler;
#[path = "music_management/library.rs"]
mod library;
#[path = "music_management/song.rs"]
mod song;

fn main() {
    // Setup account
    let mut user = auth::register();

    // Handle commands
    command_handler::handler(&mut user);
}
