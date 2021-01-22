pub mod user_optional;

use actix_session::Session;
use actix_web::Error as ActixError;
use serde::Serialize;

use crate::{
    db::DbConn,
    models::{ModelError, Permission, User},
};
use thiserror::Error;

#[derive(Debug, Serialize)]
struct TemplateContextUser {
    /// The user's numeric id.
    id: i32,

    /// The user's Github login.
    login: String,

    /// List of the user's permissions.
    permissions: Vec<String>,
}

#[derive(Debug, Error)]
pub enum AuthFromSessionError {
    #[error("Could not parse uid from cookie with error {0}")]
    SessionRetrieveError(#[from] ActixError),

    #[error("Could not query the database with error {0}")]
    DbQueryError(#[from] ModelError),

    #[error("GithubUserRecord not found for user id {0}")]
    GithubUserRecordNotFound(i32),
}

fn auth_from_session(
    conn: &DbConn,
    session: &Session,
) -> Result<Option<(User, Vec<String>)>, AuthFromSessionError> {
    let uid = match session.get::<i32>("user_id")? {
        Some(uid) => uid,
        None => return Ok(None),
    };

    let user = match User::find_by_id(&conn, uid)? {
        Some(user) => user,
        None => {
            // remove the nonexistent user from the cookie, effectively
            // logging out the user
            session.remove("user_id");
            return Ok(None);
        }
    };

    let permissions = Permission::find_by_user_id(&conn, uid)?
        .iter()
        .map(|p| p.name.clone())
        .collect();

    return Ok(Some((user, permissions)));
}
