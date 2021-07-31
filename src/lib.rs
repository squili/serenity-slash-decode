//! Simple and easy to use slash command abstraction for [Serenity]
//!
//! Abstractions:
//! - Puts all arguments of a slash command into a map with helper functions for easy argument handling
//! - Returns full path of subcommand for easy routing
//!
//! For an example, check the `examples` directory
//!
//! [Serenity]: https://docs.rs/serenity/latest/serenity/

mod errors;

pub use crate::errors::{Error, Result};
use serenity::model::channel::PartialChannel;
use serenity::model::guild::{PartialMember, Role};
use serenity::model::interactions::{
    ApplicationCommandInteractionData, ApplicationCommandInteractionDataOptionValue,
    ApplicationCommandOptionType,
};
use serenity::model::user::User;
use std::collections::HashMap;

/// Contains the values of the slash command
#[derive(Debug)]
pub struct SlashValue {
    /// The actual value
    inner: Option<ApplicationCommandInteractionDataOptionValue>,
    /// The name of the parameter; Included for error messages
    name: String,
}

impl SlashValue {
    fn get_type_name(&self) -> String {
        match self.inner.as_ref().unwrap() {
            ApplicationCommandInteractionDataOptionValue::String(_) => "String".to_string(),
            ApplicationCommandInteractionDataOptionValue::Integer(_) => "Integer".to_string(),
            ApplicationCommandInteractionDataOptionValue::Boolean(_) => "Boolean".to_string(),
            ApplicationCommandInteractionDataOptionValue::User(_, _) => "User".to_string(),
            ApplicationCommandInteractionDataOptionValue::Channel(_) => "Channel".to_string(),
            ApplicationCommandInteractionDataOptionValue::Role(_) => "Role".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    /// Returns the inner value if it is `Some`
    pub fn expect_some(&self) -> Result<ApplicationCommandInteractionDataOptionValue> {
        match self.inner.clone() {
            Some(s) => Ok(s),
            None => Err(Error::MissingValue {
                name: self.name.clone(),
            }),
        }
    }

    /// Returns the inner value if it is a `String`
    pub fn get_string(&self) -> Result<String> {
        match self.expect_some()? {
            ApplicationCommandInteractionDataOptionValue::String(s) => Ok(s),
            _ => Err(Error::WrongType {
                expected: "String".to_string(),
                found: self.get_type_name(),
                name: self.name.clone(),
            }),
        }
    }

    /// Returns the inner value if it is an `Integer`
    pub fn get_integer(&self) -> Result<i64> {
        match self.expect_some()? {
            ApplicationCommandInteractionDataOptionValue::Integer(s) => Ok(s),
            _ => Err(Error::WrongType {
                expected: "Integer".to_string(),
                found: self.get_type_name(),
                name: self.name.clone(),
            }),
        }
    }

    /// Returns the inner value if it is a `Boolean`
    pub fn get_boolean(&self) -> Result<bool> {
        match self.expect_some()? {
            ApplicationCommandInteractionDataOptionValue::Boolean(s) => Ok(s),
            _ => Err(Error::WrongType {
                expected: "Boolean".to_string(),
                found: self.get_type_name(),
                name: self.name.clone(),
            }),
        }
    }

    /// Returns the inner value if it is a tuple of `User` and `PartialMember`
    pub fn get_user(&self) -> Result<(User, Option<PartialMember>)> {
        match self.expect_some()? {
            ApplicationCommandInteractionDataOptionValue::User(u, m) => Ok((u, m)),
            _ => Err(Error::WrongType {
                expected: "User".to_string(),
                found: self.get_type_name(),
                name: self.name.clone(),
            }),
        }
    }

    /// Returns the inner value if it is a `PartialChannel`
    pub fn get_channel(&self) -> Result<PartialChannel> {
        match self.expect_some()? {
            ApplicationCommandInteractionDataOptionValue::Channel(s) => Ok(s),
            _ => Err(Error::WrongType {
                expected: "Channel".to_string(),
                found: self.get_type_name(),
                name: self.name.clone(),
            }),
        }
    }

    /// Returns the inner value if it is a `Role`
    pub fn get_role(&self) -> Result<Role> {
        match self.expect_some()? {
            ApplicationCommandInteractionDataOptionValue::Role(s) => Ok(s),
            _ => Err(Error::WrongType {
                expected: "Role".to_string(),
                found: self.get_type_name(),
                name: self.name.clone(),
            }),
        }
    }
}

/// Wrapper around `HashMap<String, SlashValue>`
pub struct SlashMap(HashMap<String, SlashValue>);

impl SlashMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    /// If `SlashMap` has value, call `SlashValue::get_string()` on it
    pub fn get_string(&self, name: &str) -> Result<String> {
        match self.0.get(name) {
            Some(s) => s.get_string(),
            None => Err(Error::MissingValue {
                name: name.to_string(),
            }),
        }
    }

    /// If `SlashMap` has value, call `SlashValue::get_integer()` on it
    pub fn get_integer(&self, name: &str) -> Result<i64> {
        match self.0.get(name) {
            Some(s) => s.get_integer(),
            None => Err(Error::MissingValue {
                name: name.to_string(),
            }),
        }
    }

    /// If `SlashMap` has value, call `SlashValue::get_boolean()` on it
    pub fn get_boolean(&self, name: &str) -> Result<bool> {
        match self.0.get(name) {
            Some(s) => s.get_boolean(),
            None => Err(Error::MissingValue {
                name: name.to_string(),
            }),
        }
    }

    /// If `SlashMap` has value, call `SlashValue::get_user()` on it
    pub fn get_user(&self, name: &str) -> Result<(User, Option<PartialMember>)> {
        match self.0.get(name) {
            Some(s) => s.get_user(),
            None => Err(Error::MissingValue {
                name: name.to_string(),
            }),
        }
    }

    /// If `SlashMap` has value, call `SlashValue::get_channel()` on it
    pub fn get_channel(&self, name: &str) -> Result<PartialChannel> {
        match self.0.get(name) {
            Some(s) => s.get_channel(),
            None => Err(Error::MissingValue {
                name: name.to_string(),
            }),
        }
    }

    /// If `SlashMap` has value, call `SlashValue::get_role()` on it
    pub fn get_role(&self, name: &str) -> Result<Role> {
        match self.0.get(name) {
            Some(s) => s.get_role(),
            None => Err(Error::MissingValue {
                name: name.to_string(),
            }),
        }
    }
}

/// Processes a `ApplicationCommandInteractionData` and returns the path and arguments
pub fn process(interaction: &ApplicationCommandInteractionData) -> (String, SlashMap) {
    // traverse
    let mut options = &interaction.options;
    let mut path = Vec::new();
    path.push(interaction.name.clone());

    loop {
        match options.get(0) {
            None => break,
            Some(option) => {
                if option.kind == ApplicationCommandOptionType::SubCommand {
                    path.push(option.name.clone());
                    options = &option.options;
                } else {
                    break;
                }
            }
        }
    }

    // map data
    let mut map = SlashMap::new();
    for option in options {
        map.0.insert(
            option.name.clone(),
            SlashValue {
                inner: option.resolved.clone(),
                name: option.name.clone(),
            },
        );
    }

    (path.join(" "), map)
}
