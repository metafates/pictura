use colored::Colorize;
use log::{Level, Metadata, Record};

pub static LOGGER: Logger = Logger;

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => {
                    eprintln!("{}: {}", "error".bold().red(), record.args())
                }
                Level::Warn => {
                    println!("{}: {}", "warning".bold().yellow(), record.args())
                }
                Level::Info => {
                    println!("{}: {}", "info".bold().cyan(), record.args())
                }
                Level::Debug => {
                    println!("{}: {}", "debug".bold().magenta(), record.args())
                }
                Level::Trace => {
                    println!("{}: {}", "trace".bold().dimmed(), record.args())
                }
            }
        }
    }
    fn flush(&self) {}
}

pub fn init_logger() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);
}