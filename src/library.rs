// Imports
use std::io::{Write, stdin, stdout};

// Song struct
pub struct Song {
    title: String,
    artist: String,
}

// Full artist library struct
pub struct Library {
    pub songs: Vec<Song>,
}

// Library functions
impl Library {
    pub fn new_song(&mut self) {
        // New account credentials
        let mut new_title = String::new();
        let mut new_artists = String::new();

        // New song title input
        print!("New song title: ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut new_title)
            .expect("Could not read new song title input!");
        let new_title = String::from(new_title.trim());

        // New song artists input
        print!("New song artist(s): ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut new_artists)
            .expect("Could not read new song artist(s) input!");
        let new_artists = String::from(new_artists.trim());

        // Add new song to library
        let new_song = Song {
            title: new_title,
            artist: new_artists,
        };
        self.songs.push(new_song);
    }

    // Print all songs
    pub fn list_all_songs(&self) {
        for song in &self.songs {
            println!("{} by {}", song.title, song.artist);
        }
    }

    // TODO: Add: searching, updating, possibly exporting
}
