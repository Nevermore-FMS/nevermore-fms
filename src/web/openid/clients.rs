use std::str::FromStr;

use oauth2::{ClientId, RedirectUrl, url::Url};
use openidconnect::core::{CoreClient};

use crate::web::openid::OpenidProvider;

pub type OidcClient = openidconnect::Client<
    openidconnect::EmptyAdditionalClaims,
    openidconnect::core::CoreAuthDisplay,
    openidconnect::core::CoreGenderClaim,
    openidconnect::core::CoreJweContentEncryptionAlgorithm,
    openidconnect::core::CoreJsonWebKey,
    openidconnect::core::CoreAuthPrompt,
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
    oauth2::StandardTokenResponse<
        openidconnect::IdTokenFields<
            openidconnect::EmptyAdditionalClaims,
            openidconnect::EmptyExtraTokenFields,
            openidconnect::core::CoreGenderClaim,
            openidconnect::core::CoreJweContentEncryptionAlgorithm,
            openidconnect::core::CoreJwsSigningAlgorithm,
        >,
        oauth2::basic::BasicTokenType,
    >,
    oauth2::StandardTokenIntrospectionResponse<
        oauth2::EmptyExtraTokenFields,
        oauth2::basic::BasicTokenType,
    >,
    oauth2::StandardRevocableToken,
    oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
    oauth2::EndpointSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointMaybeSet,
    oauth2::EndpointMaybeSet,
>;

pub fn get_oauth_client_from_id(
    openid_provider: &OpenidProvider,
    client_id: ClientId,
) -> anyhow::Result<Option<OidcClient>> {
    if client_id == ClientId::new("nevermore-fms.internal".to_string()) {
        let hostname = openid_provider.hostname();
        let scheme = if openid_provider.tls() { "https" } else { "http" };
        let callback_url: &str = &format!("{scheme}://{hostname}/logincallback");
        let client: OidcClient =
            CoreClient::from_provider_metadata(openid_provider.metadata(), client_id, None)
                .set_redirect_uri(RedirectUrl::from_url(Url::from_str(callback_url)?));
        Ok(Some(client))
    } else {
        Ok(None)
    }
}
