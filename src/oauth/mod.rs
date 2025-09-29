mod provider;

use anyhow::Result;
use provider::oauth_client_from_env;

use crate::oauth::provider::OAuthClient;

// Google OAuth client
pub fn google_client() -> Result<OAuthClient> {
    oauth_client_from_env(
        "GOOGLE",
        "https://accounts.google.com/o/oauth2/v2/auth",
        "https://oauth2.googleapis.com/token",
        "http://127.0.0.1:3000/auth/callback/google",
    )
}

// GitHub OAuth client
pub fn github_client() -> Result<OAuthClient> {
    oauth_client_from_env(
        "GITHUB",
        "https://github.com/login/oauth/authorize",
        "https://github.com/login/oauth/access_token",
        "http://127.0.0.1:3000/auth/callback/github",
    )
}
