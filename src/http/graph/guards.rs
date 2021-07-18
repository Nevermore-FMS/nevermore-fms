use async_graphql::guard::*;
use async_graphql::*;

use crate::models::user::UserType;

pub struct UserTypeGuard {
    pub user_type: UserType,
}

#[async_trait::async_trait]
impl Guard for UserTypeGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        //if ctx.data_opt::<UserType>() == Some(&self.user_type) {
            Ok(())
        //} else {
            //Err("Forbidden".into())
        //}
    }
}