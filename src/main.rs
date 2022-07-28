mod cmd;
mod common;
mod gallery;
mod generator;

fn main() {
    if let Err(e) = cmd::run() {
        println!("Error: {}", e);
    }
}
