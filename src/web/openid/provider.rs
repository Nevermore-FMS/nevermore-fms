use std::{fs::{self, File}, io::Write, path::Path};

use log::info;
use oauth2::{AuthUrl, Scope, TokenUrl};
use openidconnect::{EmptyAdditionalProviderMetadata, IssuerUrl, JsonWebKeyId, JsonWebKeySetUrl, ResponseTypes, UserInfoUrl, core::{CoreClaimName, CoreJsonWebKey, CoreJsonWebKeySet, CoreJwsSigningAlgorithm, CoreProviderMetadata, CoreResponseType, CoreSubjectIdentifierType}};
use rsa::{RsaPrivateKey, pkcs8::{DecodePrivateKey, EncodePrivateKey}, traits::PublicKeyParts};

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