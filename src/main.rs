use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = "This is jani")]
struct Cmd {
    name: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Test {
        #[arg(short, long)]
        list: bool,
    },
}
fn main() {
    let cmd = Cmd::parse();

    if let Some(name) = cmd.name.as_deref() {
        println!("Value for name: {name}");
    }

    match &cmd.command {
        Some(command) => {
            println!("SOme command: {:?}", command)
        }
        None => {}
    }
}
