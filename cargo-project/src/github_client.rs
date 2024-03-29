//! Super-simple client for interacting with the Github API. This is clearly
//! very incomplete, but it does have the few API calls that iDevGames needs to
//! function.

use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use thiserror::Error;

/// Abstraction for interfacing with Github. This encapsulates an HTTP client,
/// our OAuth application id, and client secret.
///
/// To start the OAuth workflow, first send a user to the URL generated by
/// `authorization_url`. This will allow them to log in, which will send them
/// back to a callback URL specified in Github's OAuth application settings (on
/// Github's side, there's no control of that here). From there we have an
/// "access code" which we can exchange for an "access token" with
/// `get_access_token`. This token allows us to make calls on the user's behalf,
/// so it is very secret and ought to never be shared or logged.
#[derive(Clone)]
pub struct GithubClient {
    http_client: ReqwestClient,
    /// The github client id. This one gets exposed publicly.
    client_id: String,
    /// The secret key that is known only to us on the server and to Github.
    /// Keep this one private!
    client_secret: String,
}

impl GithubClient {
    /// Configures a Reqwest client that is compatible with what Github requires
    /// of HTTP clients interacting with it. In this case, it means having a
    /// User-Agent string in the header.
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        Self {
            http_client: reqwest::ClientBuilder::new()
                // github requires that a user agent be set to use its api
                .user_agent("Rust/reqwest/iDevGames.com")
                .build()
                .unwrap(),
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
        }
    }

    /// The URL to send a user to in order to start the OAuth workflow.
    pub fn authorization_url(&self) -> String {
        format!(
            "http://github.com/login/oauth/authorize?client_id={}",
            self.client_id
        )
    }

    /// Exchange our access code for an access token.
    pub async fn get_access_token(
        &self,
        code: &str,
    ) -> Result<GetAccessTokenResponse, GithubClientError> {
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("code", code),
        ];

        let r = self
            .http_client
            .post("https://github.com/login/oauth/access_token")
            .form(&params)
            .header("Accept", "application/json")
            .send()
            .await?
            .json()
            .await?;

        Ok(r)
    }

    /// Gets a users details by their access token, which means they have logged
    /// in via oauth.
    pub async fn get_user_detail_by_access_token(
        &self,
        access_token: &str,
    ) -> Result<UserDetailResponse, GithubClientError> {
        let r = self
            .http_client
            .get("https://api.github.com/user")
            .header("Authorization", format!("token {}", access_token))
            .header("Accept", "application/json")
            .send()
            .await?
            .json()
            .await?;

        Ok(r)
    }

    /// Gets a users details by their login name, such as `mysteriouspants`.
    pub async fn get_user_detail_by_login(
        &self,
        login: &str,
    ) -> Result<UserDetailResponse, GithubClientError> {
        let r = self
            .http_client
            .get(&format!("https://api.github.com/users/{}", login))
            .header("Accept", "application/json")
            .send()
            .await?
            .json()
            .await?;

        Ok(r)
    }
}

impl std::fmt::Debug for GithubClient {
    /// This custom debug printer omits the client secret.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "GithubClient {{ http_client: {:?}, client_id: {}, \
                client_secret: REDACTED }}",
            self.http_client, self.client_id
        )
    }
}

/// The response we get back from Github with our access token, which allows us
/// to make requests to the Github API as the user. Aside from `access_token` we
/// ignore the other fields as they are not relevant to us.
#[derive(Deserialize)]
pub struct GetAccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

impl std::fmt::Debug for GetAccessTokenResponse {
    /// This custom debug printer omits the access token.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "AuthorizationResponse {{ access_token: REDACTED, token_type: \
                {}, scope: {} }}",
            self.token_type, self.scope
        )
    }
}

/// The structure we map the user details from Github onto for internal user.
///
/// Broadly speaking, these are the only fields we're truly interested in from
/// Github. The id is the most important, for it is how we can durably refer to
/// a user even if they change their alias on Github. The login pre-populates
/// a user's identity on uDevGames, and the avatar and link to their github
/// might become useful in the future, though it's not a sure thing.
#[derive(Deserialize, Debug)]
pub struct UserDetailResponse {
    pub id: i64,
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

#[derive(Debug, Error)]
pub enum GithubClientError {
    #[error("Calling Github failed with error {0}")]
    HttpError(#[from] reqwest::Error),
}
