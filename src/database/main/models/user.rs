use diesel::prelude::*;

use super::super::interface::MainDbInterface;

#[derive(Clone, HasQuery, Insertable, AsChangeset)]
#[diesel(table_name = super::super::schema::users)]
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
    pub fn get_users(&self) -> anyhow::Result<Vec<DBUser>> {
        let result = DBUser::query().load(&mut self.db_pool.get()?)?;
        Ok(result)
    }

    pub fn get_user_by_id(&self, user_id: String) -> anyhow::Result<Option<DBUser>> {
        let result = DBUser::query()
            .find(user_id)
            .get_result(&mut self.db_pool.get()?)
            .optional()?;
        Ok(result)
    }

    pub fn update_user(&self, new_user: DBUser) -> anyhow::Result<()> {
        use super::super::schema::users::dsl::*;

        diesel::update(users.find(new_user.id.clone()))
            .set(new_user)
            .execute(&mut self.db_pool.get()?)?;
        Ok(())
    }

    pub fn insert_user(&self, new_user: DBUser) -> anyhow::Result<()> {
        use super::super::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(new_user)
            .execute(&mut self.db_pool.get()?)?;
        Ok(())
    }
}
