use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error<'a> {
    WrongType {
        expected: String,
        found: String,
        name: &'a str,
    },
    MissingValue {
        name: &'a str,
    },
}

impl Display for Error<'_> {
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

impl std::error::Error for Error<'_> {}

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;
