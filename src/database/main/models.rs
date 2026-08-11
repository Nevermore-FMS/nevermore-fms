use diesel::prelude::*;

use super::interface::MainDbInterface;

#[derive(Clone, HasQuery, AsChangeset)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DBUser {
    pub id: String,
    pub username: String,
    pub full_name: String,
    pub permissions: String,
}

impl DBUser {
    pub fn permissions_value(&self) -> anyhow::Result<Vec<String>> {
        Ok(serde_json::from_str(self.permissions.as_str())?)
    }

    pub fn set_permissions_value(&mut self, new_permissions: &[String]) -> anyhow::Result<()> {
        self.permissions = serde_json::to_string(&new_permissions)?;
        Ok(())
    }
}

impl MainDbInterface {
    // User

    pub fn get_users(self) -> anyhow::Result<Vec<DBUser>> {
        let result: Vec<DBUser> = DBUser::query().load(&mut self.db_pool.get()?)?;
        Ok(result)
    }

    pub fn get_user_by_id(self, user_id: String) -> anyhow::Result<DBUser> {
        let result: DBUser = DBUser::query()
            .find(user_id)
            .get_result(&mut self.db_pool.get()?)?;
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

    pub fn set_user_permissions(
        self,
        user_id: String,
        new_permissions: &[String],
    ) -> anyhow::Result<()> {
        use super::schema::users::dsl::*;

        diesel::update(users.filter(id.eq(user_id)))
            .set(permissions.eq(serde_json::to_string(&new_permissions)?))
            .execute(&mut self.db_pool.get()?)?;
        Ok(())
    }

    pub fn update_user(self, new_user: DBUser) -> anyhow::Result<()> {
        use super::schema::users::dsl::*;

        diesel::update(users)
            .set(new_user)
            .execute(&mut self.db_pool.get()?)?;
        Ok(())
    }
}
