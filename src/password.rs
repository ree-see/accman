pub mod password {
    use thiserror;

    #[derive(Debug, Clone, PartialEq, Hash)]
    pub struct Password {
        value: String
    }
    
    #[derive(thiserror::Error, Debug)]
    enum PasswordCreationError {
        #[error("Password must be 8 characters long")]
        TooShort(String),
        #[error("Password must contain [a-z, A-Z, 0-9] only")]
        NoSpecialChars(String),
        #[error("Passwords do not match")]
        DoesNotMatch(String),
    }

    impl Password {
        pub fn new(input: String) -> Self {
            Self { value: input }
        }

        pub fn get_password(&self) -> String {
            self.value.clone()
        }

        pub fn set_password(&mut self, new_password: String) {
            self.value = new_password        
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

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_new_password() {
            assert_eq!(Password { value: "hello".into() }, Password::new("hello".into()));
        }

        #[test]
        fn test_get_password() {
            let password = Password::new("hello".into());
            assert_eq!("hello".to_string(), password.get_password());
        }

        #[test]
        fn test_set_password() {
            let mut password = Password::new(String::from("hello"));
            let _ = &password.set_password("newhello".into());
            assert_eq!(Password::new("newhello".into()), password);
        }

        #[test]
        fn test_password_from_string() {
            let password = Password::from("hello");
            assert_eq!(Password::new("hello".into()), password.clone());
        }

        #[test]
        fn test_password_length() {
            let password = Password::from("hello");
            assert_eq!(password.get_password().len(), 5);
        }
    }
}
