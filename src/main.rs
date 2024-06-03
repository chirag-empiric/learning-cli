use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(default_value = "darwin")]
    name: String,
}

#[derive(Subcommand)]
enum Commands {
    Add(AddArgs),
}

#[derive(Args)]
struct AddArgs {
    n1: u8,
    n2: u8,
}
fn main() {
    let cli = Cli::parse();
    println!("Name is: {}", cli.name);

    match &cli.command {
        Commands::Add(args) => {
            println!("Addition is: {:?}", args.n1 + args.n2);
        }
    }
}
