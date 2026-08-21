use sha2::{Digest, Sha256};
use std::fmt;

use crate::database::main::interface::MainDbInterface;

#[derive(Clone, PartialEq, Eq)]
pub struct Permission(String);

impl From<String> for Permission {
    fn from(str: String) -> Self {
        Permission(str)
    }
}

pub type Permissions = Vec<Permission>;

#[allow(non_camel_case_types)]
pub enum nevermore_fms_permissions {
    CommandField,
    HeadReferee,
    ManageSchedule,
    ManageUsers,
    ReadField,
    Referee,
}

impl fmt::Display for nevermore_fms_permissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandField => write!(f, "nevermore-fms.command-field"),
            Self::HeadReferee => write!(f, "nevermore-fms.head-referee"),
            Self::ManageSchedule => write!(f, "nevermore-fms.manage-schedule"),
            Self::ManageUsers => write!(f, "nevermore-fms.manage-users"),
            Self::ReadField => write!(f, "nevermore-fms.read-field"),
            Self::Referee => write!(f, "nevermore-fms.referee"),
        }
    }
}

impl From<nevermore_fms_permissions> for Permission {
    fn from(val: nevermore_fms_permissions) -> Self {
        Permission(val.to_string())
    }
}

pub trait AuthenticatedContext {
    fn permissions(&self) -> Permissions;
    fn token(&self) -> &str;
    fn user_context(self) -> Option<UserAuthenticatedContext>;
    fn app_context(self) -> Option<AppAuthenticatedContext>;
    fn app_on_behalf_of_user_context(self) -> Option<AppOnBehalfOfUserAuthenticatedContext>;
}

pub struct UserAuthenticatedContext {
    token: String,
    user_id: String,
    username: String,
    full_name: String,
    permissions: Permissions,
}

impl AuthenticatedContext for UserAuthenticatedContext {
    fn permissions(&self) -> Permissions {
        self.permissions.clone()
    }
    fn token(&self) -> &str {
        &self.token
    }
    fn user_context(self) -> Option<UserAuthenticatedContext> {
        Some(self)
    }
    fn app_context(self) -> Option<AppAuthenticatedContext> {
        None
    }
    fn app_on_behalf_of_user_context(self) -> Option<AppOnBehalfOfUserAuthenticatedContext> {
        None
    }
}

impl UserAuthenticatedContext {
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn full_name(&self) -> &str {
        &self.full_name
    }
}

pub struct AppAuthenticatedContext {
    token: String,
    app_id: String,
    permissions: Permissions,
}

impl AuthenticatedContext for AppAuthenticatedContext {
    fn permissions(&self) -> Permissions {
        self.permissions.clone()
    }
    fn token(&self) -> &str {
        &self.token
    }
    fn user_context(self) -> Option<UserAuthenticatedContext> {
        None
    }
    fn app_context(self) -> Option<AppAuthenticatedContext> {
        Some(self)
    }
    fn app_on_behalf_of_user_context(self) -> Option<AppOnBehalfOfUserAuthenticatedContext> {
        None
    }
}

impl AppAuthenticatedContext {
    pub fn app_id(&self) -> &str {
        &self.app_id
    }
}

pub struct AppOnBehalfOfUserAuthenticatedContext {
    token: String,
    app_id: String,
    user_id: String,
    username: String,
    full_name: String,
    permissions: Permissions,
}

impl AuthenticatedContext for AppOnBehalfOfUserAuthenticatedContext {
    fn permissions(&self) -> Permissions {
        self.permissions.clone()
    }
    fn token(&self) -> &str {
        &self.token
    }
    fn user_context(self) -> Option<UserAuthenticatedContext> {
        None
    }
    fn app_context(self) -> Option<AppAuthenticatedContext> {
        None
    }
    fn app_on_behalf_of_user_context(self) -> Option<AppOnBehalfOfUserAuthenticatedContext> {
        Some(self)
    }
}

impl AppOnBehalfOfUserAuthenticatedContext {
    pub fn app_id(&self) -> &str {
        &self.app_id
    }
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn full_name(&self) -> &str {
        &self.full_name
    }
}

pub type StoredAuthenticatedContext = Box<dyn AuthenticatedContext + Send + Sync>;

pub fn exchange_token(
    db_interface: &MainDbInterface,
    token: &str,
) -> anyhow::Result<Option<StoredAuthenticatedContext>> {
    let token_hash = hex::encode(Sha256::digest(token));

    let Some(token_record) = db_interface.get_active_authentication_token_by_token_hash(token_hash)?
    else {
        return Ok(None);
    };

    if token_record.target_type != "user" {
        return Ok(None);
    }

    let Some(user) = db_interface.get_user_by_id(token_record.target_id)? else {
        return Ok(None);
    };

    let permissions = user
        .permissions_value()?
        .into_iter()
        .map(Permission::from)
        .collect();

    Ok(Some(Box::new(UserAuthenticatedContext {
        token: token.to_owned(),
        user_id: user.id.clone(),
        username: user.username.clone(),
        full_name: user.full_name.clone(),
        permissions,
    })))
}
