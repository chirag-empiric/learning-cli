use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    one: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    println!("One: {:?}", cli.one);
}
