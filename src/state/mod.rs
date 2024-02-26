use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
    sync::Mutex,
};

use tracing::debug;

use self::user::{Name, User};

pub mod auth;
pub mod user;

#[derive(Default)]
pub struct AppState {
    users: Mutex<HashMap<Name, User>>,
}

impl AppState {
    pub fn add_user(&self, new_user: User) -> Result<(), AddUserError> {
        let Ok(mut guard) = self.users.lock() else {
            debug!("users mutex poisoned");
            return Err(AddUserError::Other);
        };

        let Entry::Vacant(spot) = guard.entry(new_user.name().clone()) else {
            return Err(AddUserError::AlreadyExists);
        };

        spot.insert(new_user);

        Ok(())
    }

    pub fn users(&self) -> Result<Vec<Name>, UsersError> {
        let Ok(guard) = self.users.lock() else {
            debug!("users mutex poisoned");
            return Err(UsersError::Other);
        };

        Ok(guard.keys().map(|name_ref| name_ref.clone()).collect())
    }

    pub fn user_exists(&self, name: &Name) -> Result<bool, UsersError> {
        let Ok(guard) = self.users.lock() else {
            debug!("users mutex poisoned");
            return Err(UsersError::Other);
        };

        Ok(guard.contains_key(name))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddUserError {
    AlreadyExists,
    Other,
}

impl Display for AddUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists => write!(f, "already exists"),
            Self::Other => write!(f, "other"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum UsersError {
    Other,
}

impl Display for UsersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other => write!(f, "other"),
        }
    }
}
