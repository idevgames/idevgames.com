use super::HandlerError;
use crate::{
    application_context::ApplicationContext,
    db::DbConn,
    github_client::GithubClient,
    helpers::maybe_user::MaybeUser,
    models::{GithubUserRecord, User},
};
use rocket::{
    delete, get,
    http::{Cookie, CookieJar},
    response::content::Json,
    State,
};
use serde::Serialize;

/// Describes the currently logged in user, if there is a user logged
/// in.
#[get("/session")]
pub async fn get_session(user: MaybeUser) -> Json<GetSessionOutput> {
    if let Some(u) = user.user {
        GetSessionOutput {
            user: Some(SessionIdentity {
                id: u.0.id,
                github_user_id: u.1.id,
                preferred_name: u.1.preferred_name,
                permissions: user.permissions,
            }),
        }
    } else {
        GetSessionOutput { user: None }
    }
}

#[derive(Debug, Serialize)]
pub struct GetSessionOutput {
    user: Option<SessionIdentity>,
}

#[derive(Debug, Serialize)]
pub struct SessionIdentity {
    id: u32,
    github_user_id: u32,
    preferred_name: String,
    permissions: Vec<String>,
}

/// The URL that a client should redirect the user to in order to start
/// the login process.
#[get("/session/github_authorization_url")]
pub async fn github_authorization_url(
    ctxt: State<ApplicationContext>,
) -> Json<GithubAuthorizationUrlOutput> {
    let url = ctxt.github_client.authorization_url();
    Json(GithubAuthorizationUrlOutput { url: url.into() })
}

#[derive(Debug, Serialize)]
pub struct GithubAuthorizationUrlOutput {
    url: String,
}

/// Github will redirect users to this URL on successful authentication
/// with a code. This is exchanged with our secret key for an
/// authorization, which we can use to query the Github API as that
/// user. Since we don't request any scopes the only thing we can do is
/// query our current identity, which is all we wanted to do, anyway.
#[get("/session/github_callback?<code>")]
pub async fn github_callback(
    ctxt: State<ApplicationContext>,
    cookies: &CookieJar<'_>,
    code: &str,
) -> Result<Json<GithubCallbackOutput>, super::HandlerError> {
    let conn = ctxt.db_pool.get()?;
    let user = auth_with_github(&ctxt.github_client, &conn, &code).await?;
    let cookie = Cookie::new("gh_user_id", user.id.to_string());

    cookies.add_private(cookie);

    Ok(Json(GithubCallbackOutput {}))
}

/// Anything we went to communicate back to the client on successful
/// login with Github.
#[derive(Debug, Serialize)]
pub struct GithubCallbackOutput {}

/// Authenticates with Github by exchanging the access code the user
// gave us for an access token that Github issues us. Fetches the user's
/// details from Github if they are already there, otherwise returns a
/// 404.
async fn auth_with_github(
    github_client: &GithubClient,
    conn: &DbConn,
    code: &String,
) -> Result<User, super::HandlerError> {
    let authorization = github_client.get_access_token(&code).await?;
    let user_detail = github_client
        .get_user_detail_by_access_token(&authorization.access_token)
        .await?;
    let github_user = match GithubUserRecord::find_by_id(conn, user_detail.id)? {
        Some(gu) => gu,
        None => return Err(HandlerError::NotFound),
    };
    let user = match User::find_by_id(conn, github_user.user_id)? {
        Some(u) => u,
        None => return Err(HandlerError::NotFound),
    };

    Ok(user)
}

/// Logs the user out. Pitches all the cookies we set.
#[delete("/session")]
pub async fn logout(cookies: &CookieJar<'_>) -> Json<LogoutOutput> {
    cookies.remove_private(Cookie::named("gh_user_id"));
    Json(LogoutOutput {})
}

#[derive(Debug, Serialize)]
pub struct LogoutOutput {}
