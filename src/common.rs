use std::path::PathBuf;

pub const PICTURA: &str = "pictura";
pub const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png"];


pub fn get_pictura_dir() -> PathBuf {
    PathBuf::from(format!(".{PICTURA}"))
}

pub fn get_wallpapers_dir() -> PathBuf {
    PathBuf::from("Wallpapers")
}

pub fn get_compressed_dir() -> PathBuf {
    get_pictura_dir().join("compressed")
}

pub fn get_medium_dir() -> PathBuf {
    get_pictura_dir().join("medium")
}

pub fn get_config_file() -> PathBuf {
    get_pictura_dir().join("config.toml")
}

pub fn get_mappings_file() -> PathBuf {
    get_pictura_dir().join("mappings.toml")
}

pub fn get_web_dir() -> PathBuf {
    PathBuf::from(".")
}