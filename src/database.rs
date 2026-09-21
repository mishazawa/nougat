use std::sync::{Arc, Mutex};

use anyhow::anyhow;

use crate::{objects::User, utils::Error};

pub type DatabaseHandle = Arc<Database>;

pub struct Database {
    pub users: Mutex<Vec<User>>,
}

impl Database {
    pub fn local_user(&self) -> User {
        let lock = self.users.lock().unwrap();
        lock.first().unwrap().clone()
    }

    pub fn read_user(&self, name: &str) -> Result<User, Error> {
        let db_user = self.local_user();
        if name == db_user.name {
            Ok(db_user)
        } else {
            Err(anyhow!("Invalid user {name}").into())
        }
    }
}
