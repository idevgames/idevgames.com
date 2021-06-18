use super::AuthFromRequestError;
use crate::{helpers::auth_from_request, models::{GithubUserRecord, User}};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

pub struct MaybeUser {
    pub user: Option<(User, GithubUserRecord)>,
    pub permissions: Vec<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MaybeUser {
    type Error = AuthFromRequestError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match auth_from_request(req) {
            Ok(Some((user, github_user, permissioms))) => Outcome::Success(MaybeUser {
                user: Some((user, github_user)),
                permissions: permissioms,
            }),
            Ok(None) => Outcome::Success(MaybeUser {
                user: None,
                permissions: vec![],
            }),
            Err(e) => match e {
                AuthFromRequestError::DbPoolError(_) => {
                    Outcome::Failure((Status::InternalServerError, e))
                }
                AuthFromRequestError::UserIdDecodeError(_) => {
                    Outcome::Failure((Status::BadRequest, e))
                }
                AuthFromRequestError::DbQueryError(_) => Outcome::Failure((Status::BadRequest, e)),
            },
        }
    }
}
