// Imports
use crate::song::Song;
use std::io::{Write, stdin, stdout};

// Full artist library struct
pub struct Library {
    pub songs: Vec<Song>,
}

// Library functions
impl Library {
    // Create and add new song
    pub fn new_song(&mut self) {
        // New song info
        let mut new_title = String::new();
        let mut new_artists_input = String::new();
        let mut new_artists = vec![];

        // New song title input
        print!("New song title: ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut new_title)
            .expect("Could not read new song title input!");
        let new_title = new_title.trim();

        // New song artists input
        print!("New song artist(s) (Separate by comma): ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut new_artists_input)
            .expect("Could not read new song artist(s) input!");
        
        for token in new_artists_input.split(",") {
            new_artists.push(String::from(token.trim()));
        }

        // Create new song
        let new_song = Song::from(new_title, new_artists);

        // Add new song to library
        self.songs.push(new_song);
    }

    // Print all songs
    pub fn list_all_songs(&self) {
        if !&self.songs.is_empty() {
            for song in &self.songs {
                song.print_info();
            }
        } else {
            println!("There are no songs!");
        }
    }

    // Search song by title
    pub fn search_by_title(&mut self) -> Option<&mut Song> {
        // Song title
        let mut song_title = String::new();

        // Song to search title input
        print!("Title: ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut song_title)
            .expect("Could not read song title input!");
        let song_title = String::from(song_title.trim());

        // Search for song
        for song in &mut self.songs {
            if song.title == song_title {
                song.print_info();
                return Some(song);
            }
        }
        None
    }

    // Search song by artist(s)
    pub fn search_by_artists(&mut self) -> Option<&mut Song> {
        // Song artist(s)
        let mut search_artists_input = String::new();
        let mut search_artists = vec![];

        // Song to search artist(s) input
        print!("New song artist(s) (Separate by comma): ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut search_artists_input)
            .expect("Could not read new song artist(s) input!");

        for token in search_artists_input.split(",") {
            search_artists.push(String::from(token.trim()));
        }

        // Search for song
        for song in &mut self.songs {
            if song.artists.iter().any(|x| search_artists.contains(x))
            {
                song.print_info();
                return Some(song);
            }
        }
        None
    }

    // Update existing song
    pub fn update_song(&mut self) {
        // Search type input info
        let mut search_type_input = String::new();
        let search_type;

        // Search type input
        print!("1: Search by title, 2: Search by artist(s): ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut search_type_input)
            .expect("Could not read search type input!");
        search_type = search_type_input.trim().parse().expect("Invalid input!");

        // Parse search type input and search
        match search_type {
            1 => match self.search_by_title() {
                Some(s) => s.update(),
                None => println!("Could not find song!"),
            },

            2 => match self.search_by_artists() {
                Some(s) => s.update(),
                None => println!("Could not find song!"),
            },

            _ => println!("Invalid number!"),
        };
    }
}
