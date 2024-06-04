use crate::password::{Password, PasswordCreationError};
use chrono::{DateTime, Local};
use core::fmt;
use std::collections::BTreeMap;
use thiserror;
use regex::Regex;

#[derive(Debug, thiserror::Error)]
pub enum AccountStoreError {
    #[error("An account with the app \"{0}\" does not exist")]
    AccountDoesNotExist(String),
    #[error("An account with the app \"{0}\" already exists")]
    AccountAlreadyExists(String),
}

#[derive(Debug)]
pub struct AccountStore {
    pub accounts: BTreeMap<String, Account>,
    count: u8,
}

#[derive(Debug, Clone)]
pub struct Account {
    // field app_name: a string of the apps name of the account
    app_name: String,
    // field username: maybe a string if the app uses usernames as the account identifier
    username: Option<String>,
    // field email: string representing email associated with the apps account
    email: String,
    // filed password: password associated with the account
    password: Password,
    // field createdAt: local time at which the account was created with accman
    created_at: DateTime<Local>,
}

#[derive(Debug, thiserror::Error)]
pub enum AccountValidationError {
    #[error("Invalid email")]
    InvalidEmail,
    #[error("Invalid password: {0}")]
    InvalidPassword(PasswordCreationError),
    #[error("App name must be provided")]
    NoAppName,
    #[error("Account already exists for {0}")]
    AccountAlreadyExists(String),
}

// I like this idea but idk if the best option
pub enum ModifyAccount {
    ModAppName(String),
    ModUserName(Option<String>),
    ModEmail(String),
    ModPassword(Password),
}

impl Account {
    pub fn new(app_name: String, username: Option<String>, email:String, password: Password) -> Result<Account, AccountValidationError> {
        if app_name.len() == 0 {
            return Err(AccountValidationError::NoAppName);
        } else if !Self::validate_email(email.clone()) {
            return Err(AccountValidationError::InvalidEmail);
        } else { 
            Ok(Account {
                app_name,
                username,
                email,
                password,
                created_at: Local::now(),
            })
        }
         
    }
    pub fn from_input(
        app_name: String,
        username: String,
        email: String,
        password: (String, bool),
    ) -> Result<Account, AccountValidationError> {
        if app_name.len() == 0 {
            return Err(AccountValidationError::NoAppName);
        } else if !Self::validate_email(email.clone()) {
            return Err(AccountValidationError::InvalidEmail);
        } else if !Password::validate_input(password.0.clone()).ok().unwrap() {
            let passsword_err = Password::new(password.0, password.1).unwrap_err();
            return Err(AccountValidationError::InvalidPassword(passsword_err));
        } else { 
            if username.len() == 0 {
                let username = None;
                let password = Password::new(password.0, password.1).unwrap();
                Ok(Account {
                    app_name,
                    username,
                    email,
                    password,
                    created_at: Local::now(),
                })
            } else {
                let username = Some(username);
                let password = Password::new(password.0, password.1).unwrap();
                Ok(Account {
                    app_name,
                    username,
                    email,
                    password,
                    created_at: Local::now(),
                })
            }
            
        }
    }

    pub fn name(&self) -> String {
        self.app_name.clone()
    }

    fn validate_email(email: String) -> bool {
        let pattern = r"^[\w.+-]+@\w+\.\w{2,}$";
        let regex = Regex::new(pattern).unwrap();
        regex.is_match(email.as_str())
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        (self.app_name == other.app_name)
            && (self.email == other.email)
            && (self.password == other.password)
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "App Name: {}\n\tUsername: {:?}\n\tEmail: {}\n\tPassword: {}\n\tCreated At: {}", self.app_name, self.username, self.email, self.password, self.created_at)
    }
}

impl AccountStore {
    pub fn new() -> Self {
        AccountStore {
            accounts: BTreeMap::new(),
            // count is incremented when an account is added
            count: 0,
        }
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    pub fn push(&mut self, mut account: Account) -> Result<(), AccountStoreError> {
        if self.accounts.contains_key(&account.app_name) == true {
            return Err(AccountStoreError::AccountAlreadyExists(account.app_name));
        } else {
            let app_name = &account.app_name;
            account.password.encrypt();
            self.accounts.insert(app_name.to_string(), account);
            self.count += 1;
            Ok(())
        }
    }

    // for development sake this is not a viable method
    pub fn modify_account(
        &mut self,
        app_name: String,
        new_account: Account,
    ) -> Result<(), AccountStoreError> {
        if !self.accounts.contains_key(&app_name) {
            return Err(AccountStoreError::AccountDoesNotExist(app_name));
        }
        // idk the best way to implement this function
        // option 1 create a whole new account and delete the old one
        // option 2 allow the store to has access to editing accounts
        self.delete_account(app_name)?;
        self.push(new_account)?;
        Ok(())
    }

    pub fn delete_account(&mut self, app_name: String) -> Result<(), AccountStoreError> {
        if !self.accounts.contains_key(&app_name) {
            return Err(AccountStoreError::AccountDoesNotExist(app_name));
        }
        println!("account found");
        self.accounts.remove(&app_name);
        println!("account deleted");
        self.count -= 1;
        Ok(())
    }

    // param show_password: bool
    // if true should show password in plain text; 
    // if false password should be `********`
    pub fn list_accounts(&self, show_password: bool) {
        if self.accounts.is_empty() {
            println!("No accounts in the store");
        }
        for account in self.accounts.values() {
            println!("{}", account);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_account_creation() {
        let password = Password::try_from("password").unwrap();
        let mut store = AccountStore::new();
        let account = Account::new(
            "google".into(),
            None,
            "reesee@gmail.com".into(),
            password.clone(),
        );
        store.push(account.clone());
        let mut account1 = store.accounts["google".into()].clone();
        account1.password.decrypt();
        assert_eq!(
            Account::new("google".into(), None, "reesee@gmail.com".into(), password),
            account1
        );
    }

    #[test]
    fn test_store_count() {
        let mut store = AccountStore::new();
        for i in 0..3 {
            let pass1 = Password::try_from("password1").unwrap();
            let account = Account::new(
                format!("google{}", i),
                None,
                "reesee@gmail.com".into(),
                pass1,
            );
            store.push(account.clone());
        }
        assert_eq!(store.count(), 3);
    }

    #[test]
    fn test_deleting_account() {
        let mut store = AccountStore::new();
        let account = Account::new(
            "google".into(),
            None,
            "reesee@gmail.com".into(),
            Password::generate(26).unwrap(),
        );
        store.push(account);
        store.delete_account("google".into());
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_modify_app_name() {
        let mut store = AccountStore::new();
        let mut account = Account::new(
            "ggoogle".into(),
            None,
            "reesee@gmail.com".into(),
            Password::generate(26).unwrap(),
        );
        store.push(account);
        store.modify_account(
            "google".into(),
            Account::new(
                "google1".into(),
                None,
                "reesee@gmail.com".into(),
                Password::new("password".into(), false).unwrap(),
            ),
        );
    }

    fn test_list_accounts() {
        let mut store = AccountStore::new();
        store.push(Account::new(
            "google".into(),
            None,
            "reesee@gmail.com".into(),
            Password::new("".into(), true).unwrap(),
        ));
        store.list_accounts();
        assert!(true);
    }
}
