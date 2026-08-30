pub mod clients;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use oauth2::helpers::deserialize_space_delimited_vec;
use oauth2::{ClientId, RedirectUrl};

use log::info;
use openidconnect::core::{
    CoreClaimName, CoreJsonWebKey, CoreJsonWebKeySet, CoreJwsSigningAlgorithm,
    CoreProviderMetadata, CoreResponseMode, CoreResponseType, CoreSubjectIdentifierType,
};
use openidconnect::{
    AuthUrl, EmptyAdditionalProviderMetadata, IssuerUrl, JsonWebKeyId, JsonWebKeySetUrl,
    ResponseTypes, Scope, TokenUrl, UserInfoUrl,
};
use poem::http::{Method, StatusCode};
use poem::web::{Html, Json, Query};
use poem::{IntoResponse, Response};
use rsa::RsaPrivateKey;
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use rsa::traits::PublicKeyParts;
use serde::{Deserialize, Serialize};

use crate::fmscore::FMSCore;
use crate::web::openid::clients::{OidcClient, get_oauth_client_from_id};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Scopes(#[serde(deserialize_with = "deserialize_space_delimited_vec")] Vec<Scope>);

#[derive(Clone)]
pub struct OpenidProvider {
    hostname: String,
    tls: bool,
    metadata: CoreProviderMetadata,
    jwks: CoreJsonWebKeySet,
}

impl OpenidProvider {
    // Public API -->

    pub fn hostname(&self) -> String {
        self.hostname.clone()
    }

    pub fn tls(&self) -> bool {
        self.tls
    }

    pub fn metadata(&self) -> CoreProviderMetadata {
        self.metadata.clone()
    }

    pub fn jwks(&self) -> CoreJsonWebKeySet {
        self.jwks.clone()
    }

    pub fn new(hostname: String, tls: bool, data_dir: &Path) -> anyhow::Result<Self> {
        let scheme = if tls { "https" } else { "http" };
        let issuer_url: &str = &format!("{scheme}://{hostname}/openid");
        let authorization_url: &str = &format!("{issuer_url}/authorize");
        let keys_url: &str = &format!("{issuer_url}/keys");
        let token_url: &str = &format!("{issuer_url}/token");
        let userinfo_url: &str = &format!("{scheme}://{hostname}/api/userinfo");

        let provider_metadata = CoreProviderMetadata::new(
            IssuerUrl::new(issuer_url.to_string())?,
            AuthUrl::new(authorization_url.to_string())?,
            JsonWebKeySetUrl::new(keys_url.to_string())?,
            vec![ResponseTypes::new(vec![CoreResponseType::Code])],
            vec![CoreSubjectIdentifierType::Public],
            vec![CoreJwsSigningAlgorithm::RsaSsaPssSha256],
            EmptyAdditionalProviderMetadata {},
        )
        .set_token_endpoint(Some(TokenUrl::new(token_url.to_string())?))
        .set_userinfo_endpoint(Some(UserInfoUrl::new(userinfo_url.to_string())?))
        .set_scopes_supported(Some(vec![
            Scope::new("openid".to_string()),
            Scope::new("profile".to_string()),
        ]))
        .set_claims_supported(Some(vec![
            CoreClaimName::new("sub".to_string()),
            CoreClaimName::new("aud".to_string()),
            CoreClaimName::new("exp".to_string()),
            CoreClaimName::new("iat".to_string()),
            CoreClaimName::new("iss".to_string()),
            CoreClaimName::new("name".to_string()),
        ]));

        let private_key = get_or_create_private_key(data_dir)?;
        let modulus_bytes = private_key.n().to_bytes_be();
        let exponent_bytes = private_key.e().to_bytes_be();
        let oidc_jwk = CoreJsonWebKey::new_rsa(
            modulus_bytes,
            exponent_bytes,
            Some(JsonWebKeyId::new("rsa-1".to_string())),
        );

        Ok(OpenidProvider {
            hostname,
            tls,
            metadata: provider_metadata,
            jwks: CoreJsonWebKeySet::new(vec![oidc_jwk]),
        })
    }

    // Internal API -->
}

fn get_or_create_private_key(data_dir: &Path) -> anyhow::Result<RsaPrivateKey> {
    let key_pem_path = data_dir.join("openid_privatekey.pem");

    if let Some(parent_dir) = key_pem_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    if key_pem_path.try_exists()? {
        let pem_content = fs::read_to_string(key_pem_path)?;
        let private_key = RsaPrivateKey::from_pkcs8_pem(&pem_content)?;
        Ok(private_key)
    } else {
        info!("Generating openid private key for the first time...");
        let mut rng = rand::thread_rng();
        let bit_size = 2048;

        let private_key = RsaPrivateKey::new(&mut rng, bit_size)?;
        let pem_string = private_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)?;

        let mut file = File::create(key_pem_path)?;
        file.write_all(pem_string.as_bytes())?;

        Ok(private_key)
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthorizeQuery {
    pub client_id: ClientId,
    pub response_type: ResponseTypes<CoreResponseType>,
    pub redirect_uri: RedirectUrl,
    pub scope: Scopes,
    pub response_mode: Option<CoreResponseMode>,
    // pub state: Option<String>,
    // pub nonce: Option<String>,
    // pub code_challenge: Option<String>,
}

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

#[poem::handler]
pub async fn openid_authorization_endpoint(
    fms_core: poem::web::Data<&FMSCore>,
    method: Method,
    Query(query): Query<AuthorizeQuery>,
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
