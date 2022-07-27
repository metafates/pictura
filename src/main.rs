mod cmd;
mod common;
mod gallery;

fn main() {
    if let Err(e) = cmd::run() {
        println!("Error: {}", e);
    }
}
