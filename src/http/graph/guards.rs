use async_graphql::guard::*;
use async_graphql::*;

use crate::models::user::{User, UserType};

pub struct UserTypeGuard {
    pub user_type: UserType,
}

#[async_trait::async_trait]
impl Guard for UserTypeGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let mut maybe_user = ctx.data_opt::<User>();
        if let Some(user) = maybe_user.take() {
            if user.user_type.is_above_or_equal(self.user_type) {
                Ok(())
            } else {
                Err("Forbidden".into())
            }
        } else {
            Err("Not logged in.".into())
        }
    }
}
