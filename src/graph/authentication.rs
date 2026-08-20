use async_graphql::{Context, ErrorExtensions, Guard, Result};

use crate::database::main::models::DBUser;

pub struct PermissionGuard {
    required_permission: String,
}

impl PermissionGuard {
    pub fn requires(required_permission: &str) -> Self {
        Self {
            required_permission: required_permission.to_owned(),
        }
    }
}

impl Guard for PermissionGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        // TODO Use an AuthenticationContext from a seperate mod or similar
        if let Some(user) = ctx.data_opt::<DBUser>()
            && user
                .permissions_value()?
                .contains(&self.required_permission)
        {
            Ok(())
        } else {
            Err(async_graphql::Error::new(
                "You do not have enough permissions to perform this action",
            )
            .extend_with(|_, ext| ext.set("code", "FORBIDDEN")))
        }
    }
}
