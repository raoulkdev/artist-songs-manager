use std::io::{stdin, stdout, Write};

// Song struct
pub struct Song {
    pub title: String,
    pub artist: String,
}

// Song functions
impl Song {
    // Update song info
    pub fn update(&mut self) {
        // Song new info
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

        // Set new info
        self.title = new_title;
        self.artist = new_artists;
    }
}