pub mod admin_only;
pub mod maybe_user;
pub mod snippets;

use crate::application_context::ApplicationContext;
use crate::models::{GithubUserRecord, User};
use crate::models::{ModelError, Permission};
use rocket::request::Request;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthFromRequestError {
    #[error("Could not get a connection from the pool with error {0}")]
    DbPoolError(#[from] diesel::r2d2::PoolError),

    #[error("Could not parse uid from cookie with error {0}")]
    UserIdDecodeError(#[from] ParseIntError),

    #[error("Could not query the database with error {0}")]
    DbQueryError(#[from] ModelError),
}

fn auth_from_request<'r>(
    req: &'r Request<'_>,
) -> Result<Option<(User, GithubUserRecord, Vec<String>)>, AuthFromRequestError> {
    // unwrap is okay here, if there's no pool then the entire application
    // bootstrap was wrong
    let pool = &req.rocket().state::<ApplicationContext>().unwrap().db_pool;
    let conn = pool.read().get()?;

    // pull the user out of the cookie, if it's there
    let cookies = req.cookies();
    let user_id = cookies.get_private("user_id");

    match user_id {
        Some(cookie) => {
            let value = cookie.value();
            let uid = str::parse::<i32>(value)?;
            let user = match User::find_by_id(&conn, uid)? {
                Some(user) => user,
                None => {
                    // remove the nonexistent user from the cookie,
                    // effectively logging out the user
                    cookies.remove_private(cookie);
                    return Ok(None);
                }
            };
            let github_user = match GithubUserRecord::find_by_user_id(&conn, uid)? {
                Some(github_user) => github_user,
                None => {
                    // no associated github user is a big problem
                    cookies.remove_private(cookie);
                    return Ok(None);
                }
            };

            let permissions = Permission::find_by_user_id(&conn, uid)?
                .iter()
                .map(|p| p.name.clone())
                .collect();

            return Ok(Some((user, github_user, permissions)));
        }
        None => return Ok(None),
    };
}
