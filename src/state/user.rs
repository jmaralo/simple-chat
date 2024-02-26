use std::fmt::Display;

use serde::Serialize;

use super::auth::{TemporalToken, Token};

pub struct User {
    name: Name,

    token: Token,
    temporal_token: Option<TemporalToken>,
}

impl User {
    pub fn new(name: Name, token: Token) -> Self {
        Self {
            name,

            token,
            temporal_token: None,
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
}

#[derive(Hash, Eq, PartialEq, Serialize)]
pub struct Name(String);

impl Name {
    pub fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

const MAX_NAME_LENGTH: usize = 64;

impl TryFrom<String> for Name {
    type Error = NameError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        // TODO: check for malicious imput
        if name.len() > MAX_NAME_LENGTH {
            return Err(NameError::TooLong);
        }
        Ok(Self(name))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NameError {
    TooLong,
}

impl Display for NameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong => write!(f, "too long"),
        }
    }
}
