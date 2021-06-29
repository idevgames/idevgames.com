use super::HandlerError;
use crate::{
    application_context::ApplicationContext,
    db::DbPool,
    github_client::GithubClient,
    helpers::maybe_user::MaybeUser,
    models::{GithubUserRecord, Permission, User},
};
use rocket::{
    delete, get,
    http::{Cookie, CookieJar},
    serde::json::Json,
    State,
};
use serde::{Deserialize, Serialize};

/// The concept of a session which is re-used by several calls.
/// Permissions are separated out, and even when there is no session
/// identity an empty list of permissions will be returned.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionIdentity {
    id: i32,
    github_user_id: i64,
    login: String,
}

/// Describes the currently logged in user, if there is a user logged
/// in.
#[get("/session")]
pub async fn get_session(user: MaybeUser) -> Json<GetSessionOutput> {
    if let Some(u) = user.user {
        Json(GetSessionOutput {
            user: Some(SessionIdentity {
                id: u.0.id,
                github_user_id: u.1.id,
                login: u.1.login,
            }),
            permissions: user.permissions,
        })
    } else {
        Json(GetSessionOutput {
            user: None,
            permissions: vec![],
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSessionInput {}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSessionOutput {
    user: Option<SessionIdentity>,
    permissions: Vec<String>,
}

/// The URL that a client should redirect the user to in order to start
/// the login process.
#[get("/session/github_authorization_url")]
pub async fn get_github_authorization_url(
    ctxt: &State<ApplicationContext>,
) -> Json<GetGithubAuthorizationUrlOutput> {
    let url = ctxt.github_client.authorization_url();
    Json(GetGithubAuthorizationUrlOutput { url: url.into() })
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGithubAuthorizationUrlInput {}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGithubAuthorizationUrlOutput {
    url: String,
}

/// Github will redirect users to this URL on successful authentication
/// with a code. This is exchanged with our secret key for an
/// authorization, which we can use to query the Github API as that
/// user. Since we don't request any scopes the only thing we can do is
/// query our current identity, which is all we wanted to do, anyway.
#[get("/session/github_callback?<code>")]
pub async fn github_callback(
    ctxt: &State<ApplicationContext>,
    cookies: &CookieJar<'_>,
    code: &str,
) -> Result<Json<GithubCallbackOutput>, super::HandlerError> {
    let (user, github_user) =
        auth_with_github(&ctxt.github_client, &ctxt.db_pool, code).await?;
    let permissions = Permission::find_by_user_id(&ctxt.db_pool.read().get()?, user.id)?;
    let cookie = Cookie::new("user_id", user.id.to_string());

    cookies.add_private(cookie);

    Ok(Json(GithubCallbackOutput {
        user: SessionIdentity {
            id: user.id,
            github_user_id: github_user.id,
            login: github_user.login,
        },
        permissions: permissions
            .iter()
            .map(|permission| permission.name.clone())
            .collect(),
    }))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubCallbackInput {
    code: String,
}

/// Anything we went to communicate back to the client on successful
/// login with Github.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubCallbackOutput {
    user: SessionIdentity,
    permissions: Vec<String>,
}

/// Authenticates with Github by exchanging the access code the user
// gave us for an access token that Github issues us. Fetches the user's
/// details from Github if they are already there, otherwise returns a
/// 404.
async fn auth_with_github(
    github_client: &GithubClient,
    pool: &DbPool,
    code: &str,
) -> Result<(User, GithubUserRecord), super::HandlerError> {
    let authorization = github_client.get_access_token(code).await?;
    let user_detail = github_client
        .get_user_detail_by_access_token(&authorization.access_token)
        .await?;
    let conn = pool.read().get()?;
    let github_user = match GithubUserRecord::find_by_id(&conn, user_detail.id)? {
        Some(gu) => gu,
        None => return Err(HandlerError::NotFound),
    };
    let user = match User::find_by_id(&conn, github_user.user_id)? {
        Some(u) => u,
        None => return Err(HandlerError::NotFound),
    };

    Ok((user, github_user))
}

/// Logs the user out. Pitches all the cookies we set.
#[delete("/session", data = "<_input>")]
pub async fn delete(
    cookies: &CookieJar<'_>,
    _input: Json<DeleteSessionInput>,
) -> Json<DeleteSessionOutput> {
    cookies.remove_private(Cookie::named("gh_user_id"));
    Json(DeleteSessionOutput {})
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSessionInput {}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSessionOutput {}
