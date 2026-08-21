use anyhow::Context;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::database::main::schema::authentication_tokens;

use super::super::interface::MainDbInterface;

#[derive(Clone, HasQuery, Insertable)]
#[diesel(table_name = super::super::schema::authentication_tokens)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DBAuthenticationToken {
    pub id: String,
    pub target_type: String,
    pub target_id: String,
    pub token_hash: String,
    pub created_timestamp: i64,
    pub expires_at_timestamp: i64
}

impl DBAuthenticationToken {
    pub fn created_timestamp_value(&self) -> anyhow::Result<DateTime<Utc>> {
        DateTime::from_timestamp_secs(self.created_timestamp).context("out-of-range number of seconds")
    }

    pub fn set_created_timestamp_value(&mut self, new_timestamp: DateTime<Utc>) -> anyhow::Result<()> {
        self.created_timestamp = new_timestamp.timestamp();
        Ok(())
    }

    pub fn expires_at_timestamp_value(&self) -> anyhow::Result<DateTime<Utc>> {
        DateTime::from_timestamp_secs(self.expires_at_timestamp).context("out-of-range number of seconds")
    }

    pub fn set_expires_at_timestamp_value(&mut self, new_timestamp: DateTime<Utc>) -> anyhow::Result<()> {
        self.expires_at_timestamp = new_timestamp.timestamp();
        Ok(())
    }
}

impl MainDbInterface {
    pub fn get_active_authentication_token_by_token_hash(&self, token_hash: String) -> anyhow::Result<Option<DBAuthenticationToken>> {
        let result = DBAuthenticationToken::query()
            .filter(authentication_tokens::token_hash.eq(token_hash))
            .filter(authentication_tokens::expires_at_timestamp.gt(Utc::now().timestamp()))
            .get_result(&mut self.db_pool.get()?).optional()?;
        Ok(result)
    }

    pub fn insert_authentication_token(&self, new_authentication_token: DBAuthenticationToken) -> anyhow::Result<()> {
        use super::super::schema::authentication_tokens::dsl::*;

        diesel::insert_into(authentication_tokens)
            .values(new_authentication_token)
            .execute(&mut self.db_pool.get()?)?;
        Ok(())
    }
}
