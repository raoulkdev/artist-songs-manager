# Artist Songs Manager

A Rust-based CLI application for managing artists and their songs. I created this project to learn how to to organize, search, and manipulate song data.

## Features

- **Artist Authentication**: Username and password.
- **Song Management**: Add and update songs
- **Search & Filter**: Quickly search for songs by title or artist.
- **CLI Interface**: Interact with the manager via a command-line interface.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70+ recommended)
- Cargo (comes with Rust)

### Building

Clone the repository:

```sh
git clone https://github.com/raoulkdev/artist-songs-manager.git
cd artist-songs-manager
```

Build the project:

```sh
cargo build
```

### Running

To run the application:

```sh
cargo run
```

## Usage

The application provides a set of commands for managing artists and songs. Here are a few examples:

```sh
# Create new account
New username: drake
New password : {hidden}

# Add a new song
artist_mngr -> new
New song title: Passionfruit
New song artist(s): Drake

# List all songs
artist_mngr -> ls_songs
Passionfruit by Drake

# Search for song by title
artist_mngr -> search_t
Title: Passionfruit
Passionfruit by Drake

```


## Contributing

Contributions are welcome! Please open issues or pull requests for any improvements, features, or bug fixes.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/YourFeature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin feature/YourFeature`)
5. Open a pull request

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgements

- Built with [Rust](https://www.rust-lang.org/)
- Inspired by music managers and open-source CLI tools

---
