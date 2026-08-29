use openidconnect::core::{
    CoreClaimName, CoreJwsSigningAlgorithm, CoreProviderMetadata, CoreResponseType,
    CoreSubjectIdentifierType,
};
use openidconnect::{
    AuthUrl, EmptyAdditionalProviderMetadata, IssuerUrl, JsonWebKeySetUrl, ResponseTypes, Scope,
    TokenUrl, UserInfoUrl,
};

pub fn provider_metadata() -> anyhow::Result<CoreProviderMetadata> {
    let hostname: &str = "fms.nevermore";
    let issuer_url: &str = &format!("http://{hostname}/openid");
    let authorization_url: &str = &format!("{issuer_url}/authorize");
    let keys_url: &str = &format!("{issuer_url}/keys");
    let token_url: &str = &format!("{issuer_url}/token");
    let userinfo_url: &str = &format!("http://{hostname}/api/userinfo");

    let provider_metadata = CoreProviderMetadata::new(
        IssuerUrl::new(issuer_url.to_string())?,
        AuthUrl::new(authorization_url.to_string())?,
        JsonWebKeySetUrl::new(keys_url.to_string())?,
        vec![
            ResponseTypes::new(vec![CoreResponseType::Code]),
            ResponseTypes::new(vec![CoreResponseType::Token, CoreResponseType::IdToken]),
        ],
        vec![CoreSubjectIdentifierType::Pairwise],
        vec![CoreJwsSigningAlgorithm::RsaSsaPssSha256],
        EmptyAdditionalProviderMetadata {},
    )
    // Specify the token endpoint (required for the code flow).
    .set_token_endpoint(Some(TokenUrl::new(token_url.to_string())?))
    // Recommended: support the user info endpoint.
    .set_userinfo_endpoint(Some(UserInfoUrl::new(userinfo_url.to_string())?))
    // Recommended: specify the supported scopes.
    .set_scopes_supported(Some(vec![
        Scope::new("openid".to_string()),
        Scope::new("profile".to_string()),
    ]))
    // Recommended: specify the supported ID token claims.
    .set_claims_supported(Some(vec![
        // Providers may also define an enum instead of using CoreClaimName.
        CoreClaimName::new("sub".to_string()),
        CoreClaimName::new("aud".to_string()),
        CoreClaimName::new("exp".to_string()),
        CoreClaimName::new("iat".to_string()),
        CoreClaimName::new("iss".to_string()),
        CoreClaimName::new("name".to_string()),
    ]));

    Ok(provider_metadata)
}

#[poem::handler]
pub fn openid_configuration_endpoint() -> poem::web::Json<CoreProviderMetadata> {
    poem::web::Json(provider_metadata().unwrap())
}
