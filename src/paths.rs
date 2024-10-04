use std::path::PathBuf;

pub fn config_dir() -> PathBuf {
    dirs::config_dir().unwrap().join("tsunami")
}
pub fn config_file() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn create_dir_if_not_exists(path: &PathBuf) {
    if !path.exists() {
        std::fs::create_dir_all(path).expect("Failed to create config directory");
    }
}
