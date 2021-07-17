use async_graphql::*;

use crate::application::ThreadSafeApplication;
use crate::models::plugin::Plugin;
use crate::models::user::User;

#[derive(Default)]
pub struct NodeQuery;

#[Object]
impl NodeQuery {
    async fn node<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: ID
    ) -> Result<Node> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let db = app.read().await.database.clone();
        let (type_name, id) = decode_id(id)?;
        match type_name.as_str() {
            "Plugin" => {
                Ok(Node::Plugin(Plugin::get(db, id).await?))
            }
            "User" => {
                Ok(Node::User(User::get(db, id).await?))
            }
            _ => {
                Err(Error::new("unknown type_name"))
            }
        }
    }
}

#[derive(Interface)]
#[graphql(
    field(name = "id", type = "ID"),
)]
pub enum Node {
    Plugin(Plugin),
    User(User),
}

pub fn decode_id(id: ID) -> anyhow::Result<(String, String)> {
    let b64_id = id.to_string();
    let id_bytes = base64::decode(b64_id)?;
    let id_string = String::from_utf8(id_bytes)?;
    let parts: Vec<&str> = id_string.split("|").collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("incorrectly formatted id."))
    };
    Ok((parts[0].to_string(), parts[1].to_string()))
}