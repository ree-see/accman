use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use hex;
use rand::distributions::Alphanumeric;
#[allow(unused)]
use rand::Rng;
use thiserror;

const KEY_STR: &[u8; 32] = &[42; 32];

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Password {
    value: String,
    is_encrypted: bool,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum PasswordCreationError {
    #[error("Password must be 8 characters long")]
    TooShort,
    #[error("Password cannot be longer than 256 characters long")]
    TooLong,
    #[error("Password must contain [a-z, A-Z, 0-9] only")]
    NoSpecialChars,
}

impl Password {
    pub fn new(input: String, generate: bool) -> Result<Password, PasswordCreationError> {
        if input.len() < 8 {
            Err(PasswordCreationError::TooShort)
        } else if input.len() > 256 {
            Err(PasswordCreationError::TooLong)
        } else if !input.chars().all(char::is_alphanumeric) {
            Err(PasswordCreationError::NoSpecialChars)
        } else if generate {
            Ok(Self::generate(26)?)
        } else {
            Ok(Self { value: input, is_encrypted: false, })
        }
    }

    pub fn get_password(&self) -> String {
        self.value.clone()
    }

    pub fn set_password(&mut self, new_password: String) -> Result<(), PasswordCreationError> {
        if new_password.len() < 8 {
            Err(PasswordCreationError::TooShort)
        } else if new_password.len() > 256 {
            Err(PasswordCreationError::TooLong)
        } else if !new_password.chars().all(char::is_alphanumeric) {
            Err(PasswordCreationError::NoSpecialChars)
        } else {
            self.value = new_password;
            Ok(())
        }
    }

    fn len(&self) -> usize {
        self.value.len()
    }

    pub fn generate(length: usize) -> Result<Password, PasswordCreationError> {
        println!("Password is being generated");
        let value: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();
        println!("Password has been generated");
        Password::try_from(value.as_str())
    }

    pub fn encrypt(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        let plain_password = self.get_password();
        let key = Key::<Aes256Gcm>::from_slice(KEY_STR);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let cipher = Aes256Gcm::new(key);

        let ciphered_data = cipher
            .encrypt(&nonce, plain_password.as_bytes())
            .unwrap();

        let mut encrypted_password: Vec<u8> = nonce.to_vec();
        encrypted_password.extend_from_slice(&ciphered_data);

        self.value = hex::encode(encrypted_password);
        self.is_encrypted = true;
        Ok(())
    }

    pub fn decrypt(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let encrypted_password =
            hex::decode(self.get_password()).expect("failed to decode hex string into vec");
        let key = Key::<Aes256Gcm>::from_slice(KEY_STR);
        let (nonce_arr, ciphered_data) = encrypted_password.split_at(12);
        let nonce = Nonce::from_slice(nonce_arr);
        let cipher = Aes256Gcm::new(key);

        let plain_password = cipher
            .decrypt(nonce, ciphered_data)
            .unwrap();
        self.value =
            String::from_utf8(plain_password).unwrap();
        self.is_encrypted = false;
        Ok(())
    }
}

impl TryFrom<&str> for Password {
    type Error = PasswordCreationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 8 {
            Err(PasswordCreationError::TooShort)
        } else if value.len() > 256 {
            Err(PasswordCreationError::TooLong)
        } else if !value.chars().all(char::is_alphanumeric) {
            Err(PasswordCreationError::NoSpecialChars)
        } else {
            Ok(Self {
                value: value.into(),
                is_encrypted: false,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_password() {
        assert_eq!(
            Password {
                value: "password".into(),
                is_encrypted: false,
            },
            Password::new("password".into(), false).unwrap()
        );
    }

    #[test]
    fn test_get_password() {
        let password = Password::new("password".into(), false).unwrap();
        assert_eq!("password".to_string(), password.get_password());
    }

    #[test]
    fn test_set_password() {
        let mut password = Password::new(String::from("password"), false).unwrap();
        let _ = &password.set_password("Password".into());
        assert_eq!(Password::new("Password".into(), false).unwrap(), password);
    }

    #[test]
    fn test_password_from_string() {
        let password = Password::try_from("password").unwrap();
        assert_eq!(
            Password::new("password".into(), false).unwrap(),
            password.clone()
        );
    }

    #[test]
    fn test_password_length() {
        let password = Password::try_from("password").unwrap();
        assert_eq!(password.get_password().len(), 8);
    }

    #[test]
    fn test_try_new_password_from_string() {
        let password = Password::try_from("password").unwrap();
        assert_eq!(Password::try_from("password").unwrap(), password);
    }

    #[test]
    fn test_password_validation() {
        let short_password = Password::try_from("pass").unwrap_err();
        assert_eq!(short_password, PasswordCreationError::TooShort);

        let mut long_password = "".to_string();
        for _ in 0..=256 {
            long_password.push('a');
        }
        let long_password = Password::new(long_password, false).unwrap_err();
        assert_eq!(long_password, PasswordCreationError::TooLong);

        let special_char_password = Password::try_from("*(&**)&&&^");
        assert_eq!(
            special_char_password,
            Err(PasswordCreationError::NoSpecialChars)
        );
    }

    #[test]
    fn test_encrypt_password() {
        let mut password = Password::try_from("password").unwrap();
        let _ = password.encrypt();
        let encrypted_password = password.get_password();
        let _ = password.decrypt();
        let decrypted_password = password.get_password();
        println!("encrypted_password: {}", encrypted_password);
        assert_ne!(encrypted_password, decrypted_password);
    }

    #[test]
    fn test_is_encrypted_toggle() {
        let mut password = Password::try_from("password").unwrap();
        let _ = password.encrypt();
        assert_eq!(password.is_encrypted, true);
        let _ = password.decrypt();
        assert_eq!(password.is_encrypted, false);
    }
}
