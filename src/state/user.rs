use std::fmt::Display;

use serde::Serialize;

#[derive(Hash, Eq, PartialEq, Serialize)]
pub struct UserName(String);

impl UserName {
    pub fn clone(&self) -> Self {
        UserName(self.0.clone())
    }
}

impl From<String> for UserName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
