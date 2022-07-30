use serde::{Deserialize, Serialize};
use crate::common::capitalize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub(crate) title: String,
    pub(crate) dark_theme_support: bool,
    pub(crate) animations: bool,
    pub(crate) remote: Option<String>
}

impl Default for Config {
    fn default() -> Self {
        let username = match std::env::var("USER") {
            Ok(username) => capitalize(username.as_str()),
            Err(_) => "Anon".to_string(),
        };

        Self {
            title: format!("{}'s Wallpapers", username),
            dark_theme_support: false,
            animations: false,
            remote: None
        }
    }
}
