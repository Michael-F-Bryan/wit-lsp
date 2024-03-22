use clap::Parser;

fn main() {
    let _ = Cmd::parse();
    todo!();
}

#[derive(Parser, Debug)]
#[command(author, version)]
enum Cmd {}
