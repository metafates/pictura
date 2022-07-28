use log::error;

mod cmd;
mod common;
mod gallery;
mod generator;
mod logger;

fn main() {
    logger::init_logger();

    if let Err(e) = cmd::run() {
        error!("{}", e);
    }
}
