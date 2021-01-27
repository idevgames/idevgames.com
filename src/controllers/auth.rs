use actix_session::Session;
use actix_web::{http::header::ContentType, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    application_context::ApplicationContext,
    db::DbConn,
    github_client::GithubClient,
    models::{GithubUserRecord, User},
};

use super::HandlerError;

/// Presents the login page. This is a simple page with a link to Github.com
/// which is where users start the authorization process. Other OAuth providers
/// may be supported in the future... but don't count on it.
pub fn login(ctxt: web::Data<ApplicationContext>) -> HttpResponse {
    #[derive(Serialize)]
    struct Context {
        github_oauth_url: String,
        suppress_auth_controls: bool,
    }

    let context = Context {
        github_oauth_url: ctxt.github_client.authorization_url(),
        suppress_auth_controls: true,
    };

    HttpResponse::Ok()
        .set(ContentType::html())
        .body(ctxt.render_template("login.html.tera", &context))
}

#[derive(Debug, Deserialize)]
pub struct GithubCallbackQueryParams {
    code: String,
}

/// Github will redirect users to this URL on successful authentication with a
/// code. This is exchanged with our secret key for an authorization, which we
/// can use to query the Github API as that user. Since we don't request any
/// scopes the only thing we can do is query our current identity, which is all
/// we wanted to do, anyway.
pub async fn github_callback(
    ctxt: web::Data<ApplicationContext>,
    session: Session,
    query_params: web::Query<GithubCallbackQueryParams>,
) -> Result<HttpResponse, super::HandlerError> {
    let conn = ctxt.db_pool.get()?;
    let code = &query_params.code;
    let user = auth_with_github(&ctxt.github_client, &conn, &code).await?;

    session.set("user_id", user.id)?;

    Ok(HttpResponse::PermanentRedirect()
        .header("Location", "/")
        .finish())
}

/// Authenticates with Github by exchanging the access code the user gave us for
/// an access token that Github issues us. Fetches the user's details from
/// Github if they are already there, otherwise returns a 404.
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
pub async fn logout(ctxt: web::Data<ApplicationContext>, session: Session) -> HttpResponse {
    session.clear();
    session.purge();

    #[derive(Debug, Serialize)]
    struct Context {}

    let context = Context {};

    HttpResponse::Ok()
        .set(ContentType::html())
        .body(ctxt.render_template("logout.html.tera", &context))
}
