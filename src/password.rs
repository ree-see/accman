// use thiserror;

pub mod password {

    #[derive(Debug, Clone, PartialEq, Hash)]
    pub struct Password {
        value: String
    }
    //
    // #[derive(thiserror::Error, Debug)]
    // enum PasswordCreationError {
    //     TooShort(String),
    //     NoSpecialChars(String),
    //     NoCapLetters(String),
    //     DoesNotMatch(String),
    // }

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
            &password.set_password("newhello".into());
            assert_eq!(Password::new("newhello".into()), password);
        }
    }
}
