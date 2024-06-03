use chrono::Local;
use clap::{Parser, Subcommand};
use randomizer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct UserPass {
    username: String,
    password: String,
    created_at: String,
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    func: Ops,
}

#[derive(Subcommand, Debug)]
enum Ops {
    Create,
    Read,
}

fn main() {
    let args = Args::parse();

    match args.func {
        Ops::Create => {
            create_new();
        }
        Ops::Read => {
            let data = read_old();
            println!("User: {:?}", data.unwrap());
        }
    }
}

fn read_old() -> Result<UserPass, String> {
    let mut size = "".to_string();
    println!("Enter Username: ");

    std::io::stdin().read_line(&mut size).unwrap();

    let username = size.trim().to_string();
    let file_data = std::fs::read_to_string("./src/demo.json").unwrap();
    let file_vec: Vec<UserPass> = serde_json::from_str(&file_data).unwrap_or_else(|_| Vec::new());

    for item in file_vec {
        if item.username == username {
            return Ok(item);
        }
    }

    return Err("User not found".to_string());
}

fn create_new() {
    let mut size = "".to_string();
    println!("Enter Username: ");

    std::io::stdin().read_line(&mut size).unwrap();

    let username = size.trim().to_string();
    let password = randomizer::Randomizer::ALPHANUMERIC(8)
        .string()
        .unwrap()
        .to_string();
    let created_at = Local::now().naive_utc().to_string();

    let new_entry = UserPass {
        username,
        password,
        created_at: created_at,
    };

    println!("New userpass: {:?}", new_entry);

    let old_file_content =
        std::fs::read_to_string("./src/demo.json").unwrap_or_else(|_| "[]".to_string());

    let mut user_passes: Vec<UserPass> =
        serde_json::from_str(&old_file_content).unwrap_or_else(|_| Vec::new());

    user_passes.push(new_entry);
    let _ = std::fs::write(
        "./src/demo.json",
        serde_json::to_string(&user_passes).unwrap(),
    );
}
