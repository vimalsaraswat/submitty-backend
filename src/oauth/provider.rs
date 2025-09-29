use anyhow::{Context, Result};
use oauth2::basic::{BasicClient, BasicErrorResponseType, BasicTokenType};
use oauth2::{
    AuthUrl, ClientId, ClientSecret, EmptyExtraTokenFields, EndpointNotSet, EndpointSet,
    RedirectUrl, RevocationErrorResponseType, StandardErrorResponse, StandardRevocableToken,
    StandardTokenIntrospectionResponse, StandardTokenResponse, TokenUrl,
};

pub type OAuthClient = oauth2::Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

// Create an OAuth client from environment variables
pub fn oauth_client_from_env(
    provider: &str,
    auth_url: &str,
    token_url: &str,
    default_redirect: &str,
) -> Result<OAuthClient, anyhow::Error> {
    let client_id = std::env::var(format!("{}_CLIENT_ID", provider))
        .with_context(|| format!("Missing {}_CLIENT_ID", provider))?;
    let client_secret = std::env::var(format!("{}_CLIENT_SECRET", provider))
        .with_context(|| format!("Missing {}_CLIENT_SECRET", provider))?;
    let redirect_url = std::env::var(format!("{}_REDIRECT_URL", provider))
        .unwrap_or_else(|_| default_redirect.to_string());

    let client = BasicClient::new(ClientId::new(client_id.to_string()))
        .set_client_secret(ClientSecret::new(client_secret.to_string()))
        .set_auth_uri(AuthUrl::new(auth_url.to_string()).unwrap())
        .set_token_uri(TokenUrl::new(token_url.to_string()).unwrap())
        .set_redirect_uri(RedirectUrl::new(redirect_url.to_string()).unwrap());

    Ok(client)
}
