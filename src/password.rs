#[allow(unused)]

use thiserror;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Password {
    value: String,
}

#[derive(thiserror::Error, Debug)]
enum PasswordCreationError {
    #[error("Password must be 8 characters long")]
    TooShort,
    #[error("Password cannot be longer than 256 characters long")]
    TooLong,
    #[error("Password must contain [a-z, A-Z, 0-9] only")]
    NoSpecialChars,
}

impl Password {
    pub fn new(input: String) -> Result<Password, PasswordCreationError> {
        if input.len() < 8 {
            Err(PasswordCreationError::TooShort)
        } else if input.len() > 256 {
            Err(PasswordCreationError::TooLong)
        } else if input.chars().all(char::is_alphanumeric) {
            Err(PasswordCreationError::NoSpecialChars)
        } else {
            Ok(Self { value: input })
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
        } else if new_password.chars().all(char::is_alphanumeric) {
            Err(PasswordCreationError::NoSpecialChars)
        } else {
            self.value = new_password;
            Ok(())
        }    
    }
        
    pub fn len(&self) -> usize {
        self.value.len()
    }
}

impl From<&str> for Password {
    fn from(value: &str) -> Self {
        Self { value: value.into() }
    }
}

impl TryFrom<&str> for Password {
    type Error = PasswordCreationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 8 {
            Err(PasswordCreationError::TooShort)
        } else if value.len() > 256 {
            Err(PasswordCreationError::TooLong)
        } else if value.chars().all(char::is_alphanumeric) {
            Err(PasswordCreationError::NoSpecialChars)
        } else {
            Ok(Self { value: value.into() })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_password() {
        assert_eq!(Password { value: "password".into() }, Password::new("password".into()).unwrap());
    }

    #[test]
    fn test_get_password() {
        let password = Password::new("password".into()).unwrap();
        assert_eq!("password".to_string(), password.get_password());
    }

    #[test]
    fn test_set_password() {
        let mut password = Password::new(String::from("password")).unwrap();
        let _ = &password.set_password("Password".into());
        assert_eq!(Password::new("Password".into()).unwrap(), password);
    }

    #[test]
    fn test_password_from_string() {
        let password = Password::from("password");
        assert_eq!(Password::new("password".into()).unwrap(), password.clone());
    }

    #[test]
    fn test_password_length() {
        let password = Password::from("password");
        assert_eq!(password.get_password().len(), 8);
    }

    #[test]
    fn test_try_new_password_from_string() {
        let password = Password::try_from("password").unwrap();
        assert_eq!(Password::from("password").unwrap(), password);
    }

    #[test]
    fn test_password_validation() {
        let short_password = Password::try_from("pass").unwrap_err();
        assert_eq!(short_password, Err(PasswordCreationError::TooShort));
        
        let long_password = "";
        for _ in 0..=256 {
            long_password.push("a");
        }
        let long_password = Password::try_from(long_password).unwrap_err();
        assert_eq!(long_password, Err(PasswordCreationError::TooLong));

        let special_char_password = Password::try_from("*(&**)&&&^");
        assert_eq!(special_char_password, Err(PasswordCreationError::NoSpecialChars));
    }
}
