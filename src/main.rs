#![allow(unused)]
mod account;
mod password;

use std::io::{self, Read, Write};
use std::str::FromStr;

use crate::account::{Account, AccountStore};
use crate::password::Password;

use account::AccountValidationError;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        #[arg(short, long)]
        gen_password: bool,
    },
    Delete {
        #[arg(short, long)]
        app_name: String,
    },
    Modify,
    List {
        #[arg(short, long)]
        password: bool,
    },
}

// a function to ask user to input the apps name, username/email, password 
fn build_account(gen_password: bool) -> Result<Account, AccountValidationError> {
    println!("Enter the details for this new account");
    // ask for app_name
    let mut app_name = String::new();
    println!("App name > ");
    io::stdin().read_line(&mut app_name).unwrap();
    let app_name = app_name.trim().to_string();

    // ask for username
    let mut username = String::new();
    println!("Username (if no username, leave it blank) > ");
    io::stdin().read_line(&mut username).unwrap();   
    let username = username.trim().to_string();

    // ask for email
    let mut email = String::new();
    println!("Email >");
    io::stdin().read_line(&mut email).unwrap();
    let email = email.trim().to_string();

    // ask for password
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim().to_string();

    let account = Account::from_input(app_name, username, email, (password, gen_password)).unwrap();
    println!("\nAccount has been create\n\n{}", account);

    // ask to confrim the account info
    let mut redo_str = String::new();
    println!("Confirm the account info is correct? y/n");
    io::stdin().read_line(&mut redo_str);
    if redo_str.trim() == "n" {
        build_account(gen_password);
    }
    println!("\nAccount \"{}\" was saved", account.name());
    Ok(account)
}

fn main() {
    let cli = Cli::parse();
    let mut store = AccountStore::new();

    store.push(Account::new(
        "google".into(),
        None,
        "reesee@gmail.com".into(),
        Password::new("password".into(), false).unwrap(),
    ).unwrap());

    match &cli.command {
        Some(Commands::List { password }) => {
            println!("Listing accounts...\n");
            match *password {
                true => { store.list_accounts(*password) },
                false => { store.list_accounts(!password) },
            }
            println!("\nDone listing accounts.");
        }
        Some(Commands::Create { gen_password }) => {
            let account = build_account(*gen_password).unwrap();
            store.push(account);
        }
        Some(Commands::Delete { app_name }) => {
            store.delete_account(app_name.to_string()).unwrap();
        }
        Some(Commands::Modify) => {
            todo!()
        }
        None => {
            panic!("no valid command provided");
        }
    }
}
