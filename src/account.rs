use chrono::{DateTime, Local};
use crate::password::Password;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct AccountStore {
    accounts: BTreeMap<String, Account>,
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

impl Account {
    fn new(app_name: String, username: Option<String>, email: String, password: Password) -> Self {
        Self {
            app_name,
            username,
            email,
            password,
            created_at: Local::now(),
        }
    }

    fn get_app_name(&self) -> String {
        self.app_name.clone()
    }

    fn get_username(&self) -> Option<String> {
       self.username.clone() 
    }

    fn get_email(&self) -> String {
        self.email.clone()
    }

    fn get_password(&self) -> Password {
        self.password.clone()
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        (self.app_name == other.app_name) && (self.email == other.email)
    }
}

impl AccountStore {
    fn new() -> Self {
        AccountStore {
            accounts: BTreeMap::new(),
            // count is incremented when an account is added
            count: 0,
        }
    }

    fn count(&self) -> u8 {
        self.count
    }

    fn push(&mut self, mut account: Account) {
        let app_name = account.get_app_name();
        account.password.encrypt();
        self.accounts.insert(app_name, account);
        self.count += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_account_creation() {
        let password = Password::try_from("password").unwrap();
        let mut store = AccountStore::new();
        let account = Account::new("google".into(), None, "reesee@gmail.com".into(), password);
        let _ = store.push(account.clone());
        assert_eq!(Account::new("google".into(), None, "reesee@gmail.com".into(), Password::try_from("password").unwrap()), store.accounts["google".into()]);
    }

    #[test]
    fn test_store_count() {
        let mut store = AccountStore::new();
        for i in 0..3 {
            let pass1 = Password::try_from("password1").unwrap();
            let account = Account::new(format!("google{}", i), None, "reesee@gmail.com".into(), pass1);
            store.push(account.clone());
        }
        assert_eq!(store.count(), 3);
    }
}
