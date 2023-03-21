use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Deserialize)]
pub struct Mpd {
    pub ip: String,
    pub port: String,
}

pub fn get_config() -> Mpd {
    let config_path: String = format!(
        "{}/.config/mpdtuirs/config.toml",
        dirs::home_dir().unwrap().display() // .config directory in users $HOME directory
    );
    let config_file: &str = &read_to_string(config_path).unwrap()[..];
    let config: Mpd = toml::from_str(config_file).unwrap();
    return config;
}
