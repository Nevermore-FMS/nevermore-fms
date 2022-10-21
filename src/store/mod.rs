use std::collections::HashMap;

use jfs::Store;

use self::user::{User, Users};

pub mod user;

pub fn create_store() -> anyhow::Result<Store> {
    let store = Store::new("data")?;

    let mut users: HashMap<String, User> = HashMap::new();

    users.insert("test".to_string(), User::new("Test".to_string(), "test".to_string(), "test".to_string()));

    let users_obj = Users{
        users
    };

    store.save_with_id(&users_obj, "users")?;
    Ok(store)
}