use clap::{Parser, Subcommand};
use std::path::Path;
use std::fs;
use std::env;
use std::error::Error;
use serde_json::json;

#[macro_use]
extern crate simple_error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate current user with password and retrieve tokens
    Auth {
        password: Option<String>,
    },

    /// Refresh existing token
    Refresh {

    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let name = env::var("USER")?;

    match &cli.command {
        Commands::Auth{ password } => {
            let pw = match password {
                None => { bail!("Password required for auth"); }
                Some(content) => { content }
            };
            let url = "http://localhost:19749/users/authenticate";
            let body = json!({
                "username": name,
                "password": pw
            });
            let client = reqwest::blocking::Client::new();
            let res = client.post(url)
                .body(body.to_string())
                .send()?;
            println!("Authenticate returned {:?}, for password {}", res, pw);
        },
        Commands::Refresh{} => {
            let homepath = env::var("HOME")?;
            let tokenpath = Path::new(&homepath).join(".umc").join("refresh");
            let token = fs::read_to_string(tokenpath)?;
            println!("Refresh Token for {} with token '{}'", name, token);
        }
    }
    Ok(())
}