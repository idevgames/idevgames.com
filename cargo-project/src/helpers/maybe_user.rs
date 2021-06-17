use rocket::{http::Status, outcome::Outcome, request::FromRequest, Request};

use crate::{
    helpers::auth_from_request,
    models::{GithubUserRecord, User},
};

use super::AuthFromRequestError;

pub struct MaybeUser {
    pub user: Option<(User, GithubUserRecord)>,
    pub permissions: Vec<String>,
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for MaybeUser {
    type Error = AuthFromRequestError;

    async fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match auth_from_request(req) {
            Ok(Some((user, permissioms))) => Outcome::Success(MaybeUser {
                user: Some(user),
                permissions: permissioms,
            }),
            Ok(None) => Outcome::Success(MaybeUser {
                user: None,
                permissions: vec![],
            }),
            Err(e) => match e {
                AuthFromRequestError::DbPoolError(e) => {
                    Outcome::Failure((Status::InternalServerError, e))
                }
                AuthFromRequestError::UserIdDecodeError(e) => {
                    Outcome::Failure((Status::BadRequest, e))
                }
                AuthFromRequestError::DbQueryError(e) => Outcome::Failure((Status::BadRequest, e)),
            },
        }
    }
}
