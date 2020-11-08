//! Boilerplate free initialization of structs from environment variables
//!
//! Usage example:
//! It is usual that programs rely on environment variables to define their behavior, especially for
//! cloud and microservies applications. Imagine you need to setup a database connection by reading the
//! following environment variables:
//!
//! * DB_HOST
//! * DB_PORT
//! * DB_USER
//! * DB_PASSWORD
//! * DB_NAME
//!
//! With this library crate, it is as easy as this:
//!
//! ```rust
//! use envconf::{Setting, Error};
//!
//! #[derive(Setting)]
//! struct DBSettings {
//!     #[conf(env = "DB_HOST", default = "localhost")]
//!     host: String,
//!     #[conf(env = "DB_PORT", default = 5432)]
//!     port: usize,
//!     #[conf(default = "myuser")]  // hardcoded setting
//!     user: String,
//!     #[conf(env = "DB_PASSWORD")] // env variable required
//!     password: String,
//!     #[conf(env = "DB_NAME", default = "mydb")]
//!     name: String,
//! }
//!
//! fn main() -> Result<(), Error<'static>> {
//!     // This env is mandatory, so it needs to be set!
//!     std::env::set_var("DB_PASSWORD", "secret");
//!
//!     // Initialize config from environment variables
//!     // Read the crate docs to check the possible Error variants
//!     let db_settings = DBSettings::init()?;
//!
//!     assert_eq!(db_settings.host, "localhost");
//!     assert_eq!(db_settings.port, 5432);
//!     assert_eq!(db_settings.user, "myuser");
//!     assert_eq!(db_settings.password, "secret");
//!     assert_eq!(db_settings.name, "mydb");
//!
//!     Ok(())
//! }
//! ```

pub use envconf_derive::Setting;

/// Possible errors returned by the `init` method
#[derive(Debug)]
pub enum Error<'a> {
    /// The environment variable was not set and there was no default. Contains the variable name
    MissingEnv(&'a str),
    /// Failed to parse the environment variable value to the field type. Contains the (variable name, value)
    EnvParse(&'a str, String),
    /// Failed to parse the default value to the field type. Contains the (field name, value)
    DefaultParse(&'a str, &'a str),
}

impl std::fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::MissingEnv(name) => writeln!(f, "Environment variable ({}) is missing", name),
            Error::EnvParse(name, value) => writeln!(
                f,
                "Failed to parse environment variable ({}) value ({})",
                name, value
            ),
            Error::DefaultParse(name, value) => writeln!(
                f,
                "Failed to parse field ({}) default value ({})",
                name, value
            ),
        }
    }
}

pub trait Setting {
    fn init<'a>() -> Result<Self, Error<'a>>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::{Error, Setting};

    #[derive(Setting)]
    struct TestSettings {
        #[conf(env = "ENVCONF_NUMBER", default = "3000")]
        pub number: usize,
        #[conf(env = "ENVCONF_STRING", default = 3000)]
        pub test: String,
        #[conf(env = "ENVCONF_ONLYENV")]
        pub onlyenv: usize,
        #[conf(default = 1000)]
        pub default: usize,
    }

    #[test]
    fn test_setting() {
        match TestSettings::init() {
            Err(Error::MissingEnv(e)) if e == "ENVCONF_ONLYENV" => (),
            _ => assert!(false),
        }

        std::env::set_var("ENVCONF_ONLYENV", "qwerty");
        match TestSettings::init() {
            Err(Error::EnvParse(n, v)) if (n == "ENVCONF_ONLYENV") && (v == "qwerty") => (),
            _ => assert!(false),
        }

        std::env::set_var("ENVCONF_ONLYENV", "50");
        match TestSettings::init() {
            Ok(s) => {
                assert_eq!(s.number, 3000);
                assert_eq!(s.test, 3000.to_string());
                assert_eq!(s.onlyenv, 50);
                assert_eq!(s.default, 1000);
            }
            _ => assert!(false),
        }

        std::env::set_var("ENVCONF_NUMBER", "9999");
        match TestSettings::init() {
            Ok(s) => {
                assert_eq!(s.number, 9999);
                assert_eq!(s.test, 3000.to_string());
                assert_eq!(s.onlyenv, 50);
                assert_eq!(s.default, 1000);
            }
            _ => assert!(false),
        }
    }
}
