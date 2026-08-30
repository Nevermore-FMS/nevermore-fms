pub mod clients;
pub mod provider;

use oauth2::{ClientId, RedirectUrl};

use openidconnect::ResponseTypes;
use openidconnect::core::{
    CoreJsonWebKeySet, CoreProviderMetadata, CoreResponseMode, CoreResponseType,
};
use poem::http::{Method, StatusCode};
use poem::web::{Html, Json, Query};
use poem::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::fmscore::FMSCore;
use crate::web::openid::clients::{OidcClient, Scopes, get_oauth_client_from_id};

#[allow(clippy::needless_pass_by_value)]
#[poem::handler]
pub fn openid_configuration_endpoint(
    fms_core: poem::web::Data<&FMSCore>,
) -> poem::web::Json<CoreProviderMetadata> {
    poem::web::Json(fms_core.openid_provider().metadata())
}

#[allow(clippy::needless_pass_by_value)]
#[poem::handler]
pub fn openid_jwks_endpoint(
    fms_core: poem::web::Data<&FMSCore>,
) -> poem::web::Json<CoreJsonWebKeySet> {
    poem::web::Json(fms_core.openid_provider().jwks())
}

#[derive(Serialize, Debug)]
pub struct OAuthErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_uri: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AuthorizeEndpointQueryParams {
    pub client_id: ClientId,
    pub response_type: ResponseTypes<CoreResponseType>,
    pub redirect_uri: RedirectUrl,
    pub scope: Scopes,
    pub response_mode: Option<CoreResponseMode>,
    // pub state: Option<String>,
    // pub nonce: Option<String>,
    // pub code_challenge: Option<String>,
}

#[poem::handler]
pub async fn openid_authorization_endpoint(
    fms_core: poem::web::Data<&FMSCore>,
    method: Method,
    Query(query): Query<AuthorizeEndpointQueryParams>,
) -> poem::Result<Response> {
    let client: Option<OidcClient> =
        get_oauth_client_from_id(&fms_core.openid_provider(), query.client_id)
            .map_err(|_| poem::Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let Some(client) = client else {
        return Ok(Json(OAuthErrorResponse {
            error: "invalid_client".to_owned(),
            error_description: Some(
                "The client identifier provided is invalid, missing, or misconfigured.".to_owned(),
            ),
            error_uri: None,
        })
        .with_status(StatusCode::BAD_REQUEST)
        .into_response());
    };

    if client.redirect_uri() != Some(&query.redirect_uri) {
        return Ok(Json(OAuthErrorResponse {
            error: "invalid_request".to_owned(),
            error_description: Some(
                "The redirect_uri MUST match the registered callback URL.".to_owned(),
            ),
            error_uri: None,
        })
        .with_status(StatusCode::BAD_REQUEST)
        .into_response());
        //TODO support multiple redirect uris
    }
    if query.response_type != ResponseTypes::new(vec![CoreResponseType::Code]) {
        return Ok(Json(OAuthErrorResponse {
            error: "unsupported_response_type".to_owned(),
            error_description: Some("Only response_type=code is supported".to_owned()),
            error_uri: None,
        })
        .into_response());
    }
    // TODO Validate scopes?
    if method == Method::GET {
        return Ok(Html("<h1>Login page here</h1>").into_response());
    }
    todo!()
}
