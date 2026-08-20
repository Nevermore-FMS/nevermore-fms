use anyhow::Context;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::database::main::schema::user_credentials;

use super::super::interface::MainDbInterface;

#[derive(Clone, HasQuery, Insertable)]
#[diesel(table_name = super::super::schema::user_credentials)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DBUserCredential {
    pub id: String,
    pub user_id: String,
    pub credential_type: String,
    pub credential_body: String,
    pub created_timestamp: i64
}

impl DBUserCredential {
    pub fn created_timestamp_value(&self) -> anyhow::Result<DateTime<Utc>> {
        DateTime::from_timestamp_secs(self.created_timestamp).context("out-of-range number of seconds")
    }

    pub fn set_created_timestamp_value(&mut self, new_timestamp: DateTime<Utc>) -> anyhow::Result<()> {
        self.created_timestamp = new_timestamp.timestamp();
        Ok(())
    }
}

impl MainDbInterface {
    pub fn get_user_credential_by_entry(self, user_id: String, credential_type: String, credential_body: String) -> anyhow::Result<Option<DBUserCredential>> {
        let result = DBUserCredential::query()
            .filter(user_credentials::user_id.eq(user_id))
            .filter(user_credentials::credential_type.eq(credential_type))
            .filter(user_credentials::credential_body.eq(credential_body))
            .get_result(&mut self.db_pool.get()?).optional()?;
        Ok(result)
    }

    pub fn get_user_credential_by_body(self, credential_type: String, credential_body: String) -> anyhow::Result<Option<DBUserCredential>> {
        let result = DBUserCredential::query()
            .filter(user_credentials::credential_type.eq(credential_type))
            .filter(user_credentials::credential_body.eq(credential_body))
            .get_result(&mut self.db_pool.get()?).optional()?;
        Ok(result)
    }

    pub fn insert_user_credential(self, new_user_credential: DBUserCredential) -> anyhow::Result<()> {
        use super::super::schema::user_credentials::dsl::*;

        diesel::insert_into(user_credentials)
            .values(new_user_credential)
            .execute(&mut self.db_pool.get()?)?;
        Ok(())
    }
}
