use std::env;

#[derive(Debug, PartialEq)]
pub enum Error {
    MissingEnv(&'static str),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

pub fn get_env(name: &'static str) -> Result<String, Error> {
    env::var(name).map_err(|_| Error::MissingEnv(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_env_variable_set() {
        env::set_var("TEST_ENV", "value");

        let result = get_env("TEST_ENV");
        assert_eq!(result, Ok("value".to_string()));

        env::remove_var("TEST_ENV");
    }

    #[test]
    fn test_get_env_variable_not_set() {
        env::remove_var("TEST_ENV");

        let result = get_env("TEST_ENV");
        assert_eq!(result, Err(Error::MissingEnv("TEST_ENV")));
    }
}
