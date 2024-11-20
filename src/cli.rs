use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run cmdj with your playlist
    Run {
        /// Name of the playlist to be played along.
        /// On omitted selects the first one defined in `.cmdj/config.json`
        playlist: Option<String>,
    },
    /// List available playlists defined in `.cmdj/config.json`
    Ls,
}

pub fn get_command() -> Commands {
    let args = CLI::parse();
    args.command
}
