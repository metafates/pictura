use std::error::Error;

use clap::{Command, command};
use colored::Colorize;

use crate::common::PICTURA;
use crate::gallery;
use crate::gallery::Config;

const INIT_CMD: &str = "init";
const INIT_CMD_TITLE_ARG: &str = "title";
const INIT_CMD_ANIMATIONS_ARG: &str = "animations";
const INIT_CMD_DARK_THEME_ARG: &str = "dark-theme";

const SYNC_CMD: &str = "sync";

/// Initialize subcommands and args.
fn init<'a>() -> Command<'a> {
    command!(PICTURA)
        .subcommand(
            Command::new(INIT_CMD)
                .about("Initialize a new gallery")
                .arg(
                    clap::Arg::with_name(INIT_CMD_TITLE_ARG)
                        .help("Name of the gallery")
                        .long(INIT_CMD_TITLE_ARG)
                        .short(INIT_CMD_TITLE_ARG.chars().next().unwrap())
                        .takes_value(true)
                )
                .arg(
                    clap::Arg::with_name(INIT_CMD_ANIMATIONS_ARG)
                        .help("Enable animations (may affect performance)")
                        .long(INIT_CMD_ANIMATIONS_ARG)
                        .short(INIT_CMD_ANIMATIONS_ARG.chars().next().unwrap())
                        .takes_value(false)
                )
                .arg(
                    clap::Arg::with_name(INIT_CMD_DARK_THEME_ARG)
                        .help("Add support for dark theme")
                        .long(INIT_CMD_DARK_THEME_ARG)
                        .short(INIT_CMD_DARK_THEME_ARG.chars().next().unwrap())
                        .takes_value(false)
                )
        )
        .subcommand(
            Command::new(SYNC_CMD)
                .about("Sync the gallery with the filesystem")
        )
}

/// Run cmd
pub fn run() -> Result<(), Box<dyn Error>> {
    let matches = init().get_matches();

    match matches.subcommand() {
        Some((INIT_CMD, sub_matches)) => {
            let mut config = Config::default();

            if sub_matches.is_present(INIT_CMD_TITLE_ARG) {
                config.title = sub_matches.value_of(INIT_CMD_TITLE_ARG).unwrap().to_string();
            }

            config.animations = sub_matches.is_present(INIT_CMD_ANIMATIONS_ARG);
            config.use_dark_theme = sub_matches.is_present(INIT_CMD_DARK_THEME_ARG);

            gallery::init(&config)?;

            println!("Initialized a new gallery called {}", config.title.green().bold());

            Ok(())
        }
        Some((SYNC_CMD, _)) => {
            gallery::sync()?;
            Ok(())
        }
        _ => {
            println!("No subcommand was used");
            Ok(())
        }
    }
}

