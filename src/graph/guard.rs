use async_graphql::{Context, ErrorExtensions, Guard, Result};

use crate::authentication::{Permission, StoredAuthenticatedContext};

pub struct PermissionGuard {
    required_permission: Permission,
}

impl PermissionGuard {
    pub fn requires(required_permission: impl Into<Permission>) -> Self {
        Self {
            required_permission: required_permission.into(),
        }
    }
}

impl Guard for PermissionGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if let Some(auth_ctx) = ctx.data_opt::<StoredAuthenticatedContext>()
            && auth_ctx.permissions().contains(&self.required_permission)
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
