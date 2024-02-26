use std::{
    fmt::Display,
    time::{Duration, Instant},
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Token(String);

impl Token {
    pub fn valid_until(self, instant: Instant) -> TemporalToken {
        TemporalToken {
            token: self,
            expiration: instant,
        }
    }

    pub fn valid_for(self, duration: Duration) -> TemporalToken {
        TemporalToken {
            token: self,
            expiration: Instant::now() + duration,
        }
    }
}

const TOKEN_LENGTH: usize = 32;

impl TryFrom<String> for Token {
    type Error = TokenError;

    fn try_from(token: String) -> Result<Self, Self::Error> {
        // TODO: check for malicious input
        if token.len() != TOKEN_LENGTH {
            return Err(TokenError::WrongLength);
        }
        Ok(Self(token))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenError {
    WrongLength,
}

impl Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongLength => write!(f, "wrong length"),
        }
    }
}

#[derive(Debug)]
pub struct TemporalToken {
    token: Token,
    expiration: Instant,
}

impl PartialEq<Token> for TemporalToken {
    fn eq(&self, other: &Token) -> bool {
        (self.expiration >= Instant::now()) && (self.token == *other)
    }
}
