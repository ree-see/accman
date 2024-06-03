#![allow(unused)]
mod account;
mod password;

use std::io::{self, Write};
use std::str::FromStr;

use crate::account::{Account, AccountStore};
use crate::password::Password;

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
    Create,
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
fn build_account() -> io::Result<Account> {
    println!("Enter the details for this new account");
    // ask for app_name
    let mut app_name = String::new();
    println!("App name > ");
    io::stdin().read_line(&mut app_name)?;

    // ask for username
    let mut username = String::new();
    println!("Username (if no username leave it blank) > ");
    io::stdin().read_line(&mut username)?;   

    // ask for email
    let mut email = String::new();
    println!("Email >");
    io::stdin().read_line(&mut email)?;

    // ask if they want a generated password
    let mut is_gen_str = String::new();
    println!("Would you like to generate a password? y/n > ");
    io::stdin().read_line(&mut is_gen_str)?;
    let is_gen_str = is_gen_str.trim();

    let mut password_str = String::new();
    if is_gen_str == "y" {
        println!("Password is being generated");
    } else if is_gen_str == "n" {
        println!("Password > ");
        io::stdin().read_line(&mut password_str)?;
    } else {
        println!("_ variant ran: [is_gen_str = {:?}]", is_gen_str);
        // need some error and ask for input again
        todo!()
    }

    let password = password_str.trim().to_string();
    let password = Password::new(password, false).unwrap();
    let account = Account::new(app_name.trim().to_string(), Some(username.trim().to_string()), email.trim().to_string(), password);
    println!("\nAccount has been create\n\n{}", account);
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
    ));

    match &cli.command {
        Some(Commands::List { password }) => {
            println!("Listing accounts...");
            match *password {
                true => { store.list_accounts(*password) },
                false => { store.list_accounts(!password) },
            }
            println!("Done listing accounts.");
        }
        Some(Commands::Create) => {
            let account = build_account().unwrap();
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
