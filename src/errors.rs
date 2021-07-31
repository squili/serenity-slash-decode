use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    WrongType {
        expected: String,
        found: String,
        name: String,
    },
    MissingValue {
        name: String,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::WrongType {
                expected,
                found,
                name,
            } => f.write_str(&*format!(
                "Wrong type in field `{}` (expected `{}`, got `{}`)",
                name, expected, found
            )),
            Error::MissingValue { name } => {
                f.write_str(&*format!("Missing value in field `{}`", name))
            }
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
