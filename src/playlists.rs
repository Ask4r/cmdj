use crate::{paths::dotdir_path, terminate};
use indexmap::IndexMap;
use serde::Deserialize;
use std::fs::{self, create_dir_all};

const CONFIG_FILE: &str = "config.json";

#[derive(Deserialize, Debug, Clone)]
pub struct Playlist {
    #[serde(default)]
    pub start: Option<String>,
    #[serde(default)]
    pub success: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}

pub fn get_playlist(playlist: String) -> Playlist {
    get_all_playlists()
        .get(&playlist)
        .unwrap_or_else(|| terminate(format!("No such playlist `{}`", playlist), 1))
        .clone()
}

pub fn default_playlist() -> Playlist {
    get_all_playlists()
        .first()
        .unwrap_or_else(|| terminate("No playlists found", 1))
        .1
        .clone()
}

pub fn get_all_playlists() -> IndexMap<String, Playlist> {
    let config_path = dotdir_path().join(CONFIG_FILE);
    if !config_path.exists() {
        create_dir_all(&config_path).unwrap_or_else(|_| {
            terminate(
                format!("Could not create `{}`", config_path.to_string_lossy()),
                1,
            )
        });
    }
    let res =
        fs::read_to_string(config_path).unwrap_or_else(|_| terminate("Failed to read config", 1));
    serde_json::from_str(&res).unwrap_or_else(|_| terminate("Failed to parse config", 1))
}

pub fn display_playlists() {
    println!(
        "{}",
        get_all_playlists()
            .iter()
            .map(|(name, playlist)| {
                let mut playlist_display = format!("{}:\n", name);
                if let Some(start) = &playlist.start {
                    playlist_display += &format!("  start: {}\n", &start);
                };
                if let Some(success) = &playlist.success {
                    playlist_display += &format!("  success: {}\n", &success);
                };
                if let Some(error) = &playlist.error {
                    playlist_display += &format!("  error: {}\n", &error);
                };
                return playlist_display;
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
}
