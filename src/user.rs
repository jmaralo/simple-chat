use std::fmt::Display;

#[derive(Hash, Eq, PartialEq)]
pub struct UserName(String);

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
