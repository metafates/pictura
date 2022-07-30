use std::path::PathBuf;

pub const PICTURA: &str = "pictura";
pub const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png"];

pub mod paths {
    use super::*;

    pub fn pictura() -> PathBuf {
        PathBuf::from(format!(".{PICTURA}"))
    }

    pub fn wallpapers() -> PathBuf {
        PathBuf::from("wallpapers")
    }

    pub fn compressed() -> PathBuf {
        pictura().join("compressed")
    }

    pub fn medium() -> PathBuf {
        pictura().join("medium")
    }

    pub fn config_file() -> PathBuf {
        pictura().join("config.toml")
    }

    pub fn pictures_file() -> PathBuf {
        pictura().join("pictures.toml")
    }

    pub fn html_file() -> PathBuf {
        PathBuf::from("index.html")
    }
}


pub fn capitalize(string: &str) -> String {
    let mut c = string.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}