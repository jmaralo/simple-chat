use std::{collections::HashMap, sync::Mutex};

pub mod user;

pub struct AppState {
    pub user_list: Mutex<HashMap<user::UserName, String>>,
}
