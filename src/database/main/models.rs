#![allow(clippy::wildcard_imports)]

use diesel::prelude::*;

use super::interface::MainDbInterface;

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: String,
    pub username: String,
    pub full_name: String,
    pub permissions: String,
}

impl MainDbInterface { // User
    pub fn get_user_by_id(self, user_id: String) -> anyhow::Result<User> {
        use super::schema::users::dsl::*;

        let result = users
            .find(user_id)
            .get_result::<User>(&mut self.db_pool.get()?)?;
        Ok(result)
    }

    pub fn set_user_username(self, user_id: String, new_username: String) -> anyhow::Result<()> {
        use super::schema::users::dsl::*;

        diesel::update(users.filter(id.eq(user_id)))
            .set(username.eq(new_username))
            .execute(&mut self.db_pool.get()?)?;
        Ok(())
    }

    pub fn set_user_fullname(self, user_id: String, new_full_name: String) -> anyhow::Result<()> {
        use super::schema::users::dsl::*;

        diesel::update(users.filter(id.eq(user_id)))
            .set(full_name.eq(new_full_name))
            .execute(&mut self.db_pool.get()?)?;
        Ok(())
    }

    pub fn set_user_permissions(self, user_id: String, new_permissions: String) -> anyhow::Result<()> {
        use super::schema::users::dsl::*;

        diesel::update(users.filter(id.eq(user_id)))
            .set(permissions.eq(new_permissions))
            .execute(&mut self.db_pool.get()?)?;
        Ok(())
    }
}
