# Accman
  A cli password management tool.
## About
  This tool is being developed because I wanted to learn rust and I am a cybersecurity student.
## Usage
`accman create <option>`
  This command creates a new account.
*Options*
- `-g | -gen_password` generates a 26 characters long random password.

`accman list`
  This command lists all accounts

`accman modify <account_name>`
  This command allows user to edit the desired account.

`accman delete <account_name>`
   This command deletes the desired account.

## Development
Right now it's not really functional because there is no persistent data actually being stored anywhere. I just have the accounts being push into BTreeMap for right now. I work a full time job and while being a full time undergrad student so. If anybody sees this repo and wants to help I wouldn't say no.
- I plan on adding a database store the accounts and adding sudo permission to run `accman`
- This is a huge maybe but I want to figure out how to actually work like a password manager on a browser
