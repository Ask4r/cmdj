use home::home_dir;
use std::path::PathBuf;

use crate::terminate::terminate;

const DOT_DIR: &str = ".cmdj";

pub fn home_path() -> PathBuf {
    home_dir().unwrap_or_else(|| terminate("Could not find HOME directory", 1))
}

pub fn dotdir_path() -> PathBuf {
    home_path().join(DOT_DIR)
}
