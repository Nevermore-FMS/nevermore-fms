use std::collections::HashMap;

use argon2::Config;
use rand::{distributions::Alphanumeric, Rng};
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Users {
    pub users: HashMap<String, User>
}

impl Users {
    pub fn new() -> Self {
        Users { users: HashMap::new() }
    }

    pub fn get_user(&self, username: String) -> Option<User> {
        return self.users.get(&username).cloned();
    }

    pub fn add_user(&mut self, user: User) -> Option<User> {
        return self.users.insert(user.username.clone(), user);
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub username: String,
    password: String
}

impl User {
    pub fn new(name: String, username: String, password: String) -> Self {
        let salt: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &Config::default()).unwrap();
        User {
            name,
            username,
            password: hash
        }
    }

    pub fn verify_password(&self, password: String) -> bool {
        let verify = argon2::verify_encoded(&self.password, password.as_bytes());
        if verify.is_err() {
            return false;
        } else {
            return verify.unwrap();
        }
    }
}