use std::io::{Write, stdin, stdout};

// Song struct
pub struct Song {
    pub title: String,
    pub artists: Vec<String>,
}

// Song functions
impl Song {
    // New empty song
    pub fn new() -> Self {
        Self {
            title: String::new(),
            artists: vec![],
        }
    }

    // New song with info
    pub fn from(title: &str, artists: Vec<String>) -> Self {
        Self {
            title: String::from(title),
            artists: artists,
        }
    }

    // Update song info
    pub fn update(&mut self) {
        // Song new info
        let mut new_title = String::new();
        let mut new_artists_input = String::new();
        let mut new_artists = vec![];

        // New song title input
        print!("New song title: ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut new_title)
            .expect("Could not read new song title input!");
        let new_title = String::from(new_title.trim());

        // New song artists input
        print!("New song artist(s) (Separate by comma): ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut new_artists_input)
            .expect("Could not read new song artist(s) input!");

        for token in new_artists_input.split(",") {
            new_artists.push(String::from(token.trim()));
        }

        // Set new info
        self.title = new_title;
        self.artists = new_artists;
    }
    
    pub fn print_info(&self) {
        print!("{} by ", self.title);
        for artist in &self.artists {
            if Some(artist) == self.artists.last(){
                println!("{artist}")
            } else {
                print!("{artist}, ")
            }
        }
    }
}
