use std::error::Error;

use clap::{Command, command};
use colored::Colorize;

use crate::gallery;
use crate::common::PICTURA;

const INIT_CMD: &str = "init";
const INIT_CMD_NAME_ARG: &str = "name";

const SYNC_CMD: &str = "sync";

/// Initialize subcommands and args.
fn init<'a>() -> Command<'a> {
    command!(PICTURA)
        .subcommand(
            Command::new(INIT_CMD)
                .about("Initialize a new gallery")
                .arg(
                    clap::Arg::with_name(INIT_CMD_NAME_ARG)
                        .help("Name of the gallery")
                        .required(true)
                        .index(1)
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
            let name = sub_matches.value_of(INIT_CMD_NAME_ARG).unwrap();

            gallery::init(name)?;
            println!("Initialized a new gallery called {}", name.green().bold());

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

