use crate::password::Password;
use chrono::{DateTime, Local};
use std::collections::BTreeMap;
use thiserror;

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

// I like this idea but idk if the best option
pub enum ModifyAccount {
    ModAppName(String),
    ModUserName(Option<String>),
    ModEmail(String),
    ModPassword(Password),
}

impl Account {
    pub fn new(
        app_name: String,
        username: Option<String>,
        email: String,
        password: Password,
    ) -> Self {
        Self {
            app_name,
            username,
            email,
            password,
            created_at: Local::now(),
        }
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        (self.app_name == other.app_name)
            && (self.email == other.email)
            && (self.password == other.password)
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

    pub fn list_accounts(&self) {
        if self.accounts.is_empty() {
            println!("No accounts in the store");
        }
        for account in self.accounts.values() {
            println!("{:?}", account);
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
