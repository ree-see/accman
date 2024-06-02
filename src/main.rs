mod account;
mod password;

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
    List,
}

fn build_account() {}
fn main() {
    let cli = Cli::parse();
    let mut store = AccountStore::new();

    store.push(Account::new(
        "google".into(),
        None,
        "reesee@gmail.com".into(),
        Password::generate(26).unwrap(),
    ));

    match &cli.command {
        Some(Commands::List) => {
            println!("Listing accounts...");
            store.list_accounts();
            println!("Done listing accounts.");
        }
        Some(Commands::Create) => {
            build_account();
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
