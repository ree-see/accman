use chrono::DateTime;
use crate::Password;

pub struct Account {
    // field app_name: a string of the apps name of the account
    app_name: String,
    // field username: maybe a string if the app uses usernames as the account identifier
    username: Option<String>,
    // field email: string representing email associated with the apps account
    email: String,
    // filed password: password associated with the account
    password: SecurePassword,
    // field createdAt: time at which the account was created with accman
    createdAt: DateTime,
}

impl Account {

    fn new(app_name: String, username: Option<String>, email: String, password: SecurePassword) -> Self {
        todo!();
    }
}
