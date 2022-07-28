mod cmd;
mod common;
mod gallery;
mod web;

fn main() {
    if let Err(e) = cmd::run() {
        println!("Error: {}", e);
    }
}
