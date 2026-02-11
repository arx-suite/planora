use actix_web::{HttpResponse, Responder, get, web};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl};
use serde::Deserialize;
use std::sync::{Arc, LazyLock};

use crate::App;
use crate::common::{ApiError, ApiResult};

#[utoipa::path(
    get,
    path = "/oauth/{provider}/start",
    tag = "Auth",
    params(
        ("provider" = OAuthProvider, description = "OAuth provider to authenticate")
    ),
    responses(
        (status = 302, description = "Redirect to OAuth provider"),
        (status = 400, description = "Invalid provider"),
        (status = 500, description = "Internal server error")
    ),
)]
#[tracing::instrument(
    name = "auth.oauth.start",
    skip_all,
    level = tracing::Level::INFO,
    fields(
        provider = %provider
    )
)]
#[get("/oauth/{provider}/start")]
async fn oauth_start(
    provider: web::Path<OAuthProvider>,
    app: web::Data<App>,
) -> Result<impl Responder, ApiError> {
    let provider = provider.into_inner();
    let api_url = app.config().api_url.clone();

    let provider_info = match provider {
        OAuthProvider::Github => &GITHUB,
        OAuthProvider::Google => &GOOGLE,
        OAuthProvider::Unknown(p) => {
            tracing::debug!(provider = %p, "Invalid provider");
            return ApiResult::to_bad_request(&format!("Invalid provider: {p}"));
        }
    };

    let client = match provider_info.client(&api_url) {
        Ok(p) => p,
        Err(err) => {
            tracing::error!("oauth client failed: {err}");
            return ApiResult::to_internal_error("internal server error");
        }
    };

    let scopes = provider_info.scopes.clone();

    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(scopes.into_iter())
        .url();

    Ok(HttpResponse::Found()
        .append_header(("Location", authorize_url.to_string()))
        .finish())
}

#[derive(Debug, Clone, PartialEq, Eq, utoipa::ToSchema)]
enum OAuthProvider {
    Github,
    Google,
    Unknown(String),
}

impl<'de> Deserialize<'de> for OAuthProvider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "github" => OAuthProvider::Github,
            "google" => OAuthProvider::Google,
            _ => OAuthProvider::Unknown(s),
        })
    }
}

impl std::fmt::Display for OAuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthProvider::Github => write!(f, "github"),
            OAuthProvider::Google => write!(f, "google"),
            OAuthProvider::Unknown(provider) => write!(f, "{provider}"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum OAuthError {
    #[error("missing or invalid OAuth environment variable: {0}")]
    Environment(#[from] std::env::VarError),

    #[error("failed to parse OAuth endpoint URL: {0}")]
    UrlParseError(#[from] url::ParseError),
}
struct OAuthProviderInfo {
    pub name: &'static str,
    pub auth_url: &'static str,
    pub token_url: &'static str,
    pub scopes: Vec<Scope>,
}

impl OAuthProviderInfo {
    pub fn client(&self, api_url: &str) -> Result<BasicClient, OAuthError> {
        let client_id = ClientId::new(std::env::var(format!("OAUTH_{}_CLIENT_ID", self.name))?);
        let client_secret =
            ClientSecret::new(std::env::var(format!("OAUTH_{}_CLIENT_SECRET", self.name))?);
        let auth_url = AuthUrl::new(self.auth_url.into())?;
        let token_url = TokenUrl::new(self.token_url.into())?;
        let redirect_url = RedirectUrl::new(format!(
            "{api_url}v1/auth/oauth/{}/callback",
            self.name.to_lowercase()
        ))?;
        tracing::info!("OAuth redirect URI: {}", redirect_url.as_str());

        let basic_client =
            BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
                .set_redirect_uri(redirect_url);

        Ok(basic_client)
    }
}

static GITHUB: LazyLock<Arc<OAuthProviderInfo>> = LazyLock::new(|| {
    Arc::new(OAuthProviderInfo {
        name: "GITHUB",
        auth_url: "https://github.com/login/oauth/authorize",
        token_url: "https://github.com/login/oauth/access_token",
        scopes: vec![Scope::new("user:email".into())],
    })
});

static GOOGLE: LazyLock<Arc<OAuthProviderInfo>> = LazyLock::new(|| {
    Arc::new(OAuthProviderInfo {
        name: "GOOGLE",
        auth_url: "https://accounts.google.com/o/oauth2/v2/auth",
        token_url: "https://oauth2.googleapis.com/token",
        scopes: vec![Scope::new("email".into()), Scope::new("profile".into())],
    })
});
