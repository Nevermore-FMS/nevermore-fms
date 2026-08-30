#![allow(clippy::unused_async)]

use async_graphql::Object;

use crate::{database::main::models::DBUser, web::graph::error::internalize_err};

pub struct GQLUser {
    pub obj_user: DBUser,
}

#[Object(name = "User")]
impl GQLUser {
    async fn id(&self) -> String {
        self.obj_user.id.clone()
    }

    async fn username(&self) -> String {
        self.obj_user.username.clone()
    }

    async fn full_name(&self) -> String {
        self.obj_user.full_name.clone()
    }

    async fn permissions(&self) -> anyhow::Result<Vec<String>> {
        self.obj_user.permissions_value().map_err(internalize_err)
    }
}