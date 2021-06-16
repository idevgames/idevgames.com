use std::num::ParseIntError;

use crate::{models::{GithubUserRecord, ModelError, User}};
use rocket::{http::Status, request::FromRequest};
use rocket::request::Request;
use rocket::request::Outcome;
use thiserror::Error;
use super::{AuthFromRequestError, auth_from_request};

pub struct AdminOnly {
    user: (User, GithubUserRecord),
    permissions: Vec<String>,
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for AdminOnly {
    type Error = AuthFromRequestError;

    async fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match auth_from_request(req) {
            Ok(Some((user, permissions))) => {
                if permissions.contains(&"admin".to_string()) {
                    Outcome::Success(AdminOnly { user, permissions })
                } else {
                    // because AdminOnly works on an API call and not on
                    // a resource there is no real danger of leaking the
                    // existence of an object. More granular permission
                    // on the resource level are the ones concerned with
                    // returning Not Found rather than Forbidden.
                    Outcome::Failure((
                        Status::Forbidden,
                        AdminOnlyError::NotAdmin,
                    ))
                }
            }
            Ok(None) => Outcome::Failure((
                Status::Unauthorized,
                AdminOnlyError::NotLoggedIn,
            )),
            Err(e) => match e {
                AuthFromRequestError::DbPoolError(e) => Outcome::Failure((
                    Status::InternalServerError,
                    AdminOnlyError::DbPoolError(e),
                )),
                AuthFromRequestError::UserIdDecodeError(e) => Outcome::Failure(
                    (Status::BadRequest, AdminOnlyError::UserIdDecodeError(e)),
                ),
                AuthFromRequestError::DbQueryError(e) => Outcome::Failure((
                    Status::BadRequest,
                    AdminOnlyError::DbQueryError(e),
                )),
            },
        }
    }
}

#[derive(Debug, Error)]
pub enum AdminOnlyError {
    #[error("The user is not an admin")]
    NotAdmin,

    #[error("No user is logged in")]
    NotLoggedIn,

    #[error("Could not get a connection from the pool with error {0}")]
    DbPoolError(#[from] diesel::r2d2::PoolError),

    #[error("Could not parse uid from cookie with error {0}")]
    UserIdDecodeError(#[from] ParseIntError),

    #[error("Could not query the database with error {0}")]
    DbQueryError(#[from] ModelError),
}
