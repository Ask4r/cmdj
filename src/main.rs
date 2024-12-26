use cli::{get_command, Commands};
use paths::home_path;
use player::Player;
use playlists::{default_playlist, display_playlists, get_playlist};
use std::{
    ffi::OsStr,
    io::{self, Write},
    process::{Command, ExitStatus},
};
use terminate::terminate;

mod cli;
mod paths;
mod player;
mod playlists;
mod terminate;

fn main() {
    let command = get_command();
    match command {
        Commands::Run { playlist, cmd } => run_cmdj(playlist, cmd),
        Commands::Ls => display_playlists(),
    };
}

fn run_cmdj(playlist_name: Option<String>, command: Option<String>) {
    let playlist = match playlist_name {
        Some(name) => get_playlist(name),
        None => default_playlist(),
    };

    let user_command = command.unwrap_or_else(get_input_command);
    let player = Player::new();

    if let Some(path) = &playlist.start {
        player.play_forever(home_path().join(path));
    }

    let status = execute_sync(&user_command);
    player.clear();

    let finish_audio = if status.success() {
        playlist.success
    } else {
        playlist.error
    };
    if let Some(path) = &finish_audio {
        player.play(home_path().join(path));
        player.sleep_until_end();
    }
}

fn get_input_command() -> String {
    println!("Enter command below");
    print!(">> ");
    io::stdout().flush().expect("Failed to display prompt");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read command");
    input
}

fn execute_sync<S: AsRef<OsStr>>(command: S) -> ExitStatus {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .expect("Failed to run shell command")
}
