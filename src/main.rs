use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    prompt: String,
    force: bool,
}

fn main() {
    println!("Hello, world!");
}
