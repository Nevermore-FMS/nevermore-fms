use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Duration, Utc};
use rand::{Rng, distributions::Alphanumeric};
use tokio::sync::RwLock;

use crate::models::{user::User, ThreadSafeDatabase};

#[derive(Clone)]
pub struct Session {
    pub username: String,
    pub expiry_time: DateTime<Utc>,
}

pub type ThreadSafeSessionStorage = Arc<RwLock<SessionStorage>>;

pub struct SessionStorage {
    token_to_session: HashMap<String, Session>,
    username_to_token: HashMap<String, String>,
}

impl SessionStorage {
    pub fn new() -> ThreadSafeSessionStorage {
        Arc::new(RwLock::new(Self {
            token_to_session: HashMap::new(),
            username_to_token: HashMap::new(),
        }))
    }

    pub fn set(&mut self, username: String, session_duration: Duration) -> String {
        self.remove(username.clone());

        let token = random_string(32);
        self.username_to_token
            .insert(username.clone(), token.clone());
        self.token_to_session.insert(
            token.clone(),
            Session {
                username,
                expiry_time: Utc::now() + session_duration,
            },
        );
        token
    }

    pub fn remove(&mut self, username: String) {
        let username_to_token = self.username_to_token.clone();
        let mut maybe_token = username_to_token.get(&username.clone());
        if let Some(token) = maybe_token.take() {
            self.username_to_token.remove(&username.clone());
            self.token_to_session.remove(token);
        };
    }

    pub async fn verify_token(
        &self,
        db: ThreadSafeDatabase,
        token: String,
    ) -> anyhow::Result<User> {
        let session = self
            .token_to_session(token)
            .ok_or(anyhow::anyhow!("session invalid"))?;

        User::get(db, session.username).await
    }

    pub fn token_to_session(&self, token: String) -> Option<Session> {
        self.token_to_session.get(&token).map(|x| x.to_owned())
    }
}

fn random_string(length: u8) -> String {
    rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(length as usize)
    .map(char::from)
    .collect()
}
