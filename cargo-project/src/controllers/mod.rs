pub mod auth;
pub mod snippets;

use crate::{github_client::GithubClientError, helpers::AuthFromRequestError};
use rocket::{Request, Response, http::{ContentType, Status}, response::{self, Responder}, serde::json::serde_json::json};
use std::{io::Cursor, num::ParseIntError};
use thiserror::Error;

/// Unified error type for most (all?) handlers. Puts all the annoying
/// boilerplate of derives into one spot with a single implementation of
/// Responder.
///
/// Note that it would be very tempting to use anyhow for this, however we
/// cannot implement Responder for it. Even if we could, inferring the http
/// status code from a Boxed error would be rather challenging.
#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("The resource was not found")]
    NotFound,

    #[error("Could not get a connection from the pool with error {0}")]
    PoolError(#[from] diesel::r2d2::PoolError),

    #[error("Failed to query the database with error {0}")]
    DatabaseError(#[from] crate::models::ModelError),

    #[error("HTTP Error {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Parse Error {0}")]
    ParseError(#[from] chrono::ParseError),

    #[error("Parse error {0}")]
    ParseIntError(#[from] ParseIntError),

    #[error("Diesel Error {0}")]
    DieselError(#[from] diesel::result::Error),

    #[error("Not authorized")]
    NotAuthorized,
}

impl HandlerError {
    fn status_code(&self) -> Status {
        match self {
            Self::DatabaseError(_) => Status::InternalServerError,
            Self::PoolError(_) => Status::InternalServerError,
            Self::HttpError(_) => Status::InternalServerError,
            Self::ParseError(_) => Status::BadRequest,
            Self::ParseIntError(_) => Status::BadRequest,
            Self::DieselError(_) => Status::InternalServerError,
            Self::NotFound => Status::NotFound,
            Self::NotAuthorized => Status::Unauthorized,
        }
    }

    fn external_message(&self) -> &str {
        match self {
            Self::NotFound => "The resource was not found",
            Self::PoolError(_) => "Unable to connect to database",
            Self::DatabaseError(_) => "Unable to query database",
            Self::HttpError(_) => "Unable to contact a remote server",
            Self::ParseError(_) => "Unable to parse date",
            Self::ParseIntError(_) => "Unable to parse int",
            Self::DieselError(_) => "Unable to query database",
            Self::NotAuthorized => "You are unauthorized",
        }
    }
}

impl<'r> Responder<'r, 'static> for HandlerError {
    fn respond_to(self, _request: &'r Request<'_>) -> response::Result<'static> {
        let status_code = self.status_code();
        let body = json!({
            "message": self.external_message()
        }).to_string();

        Response::build()
            .sized_body(body.len(), Cursor::new(body))
            .header(ContentType::JSON)
            .status(status_code)
            .ok()
    }
}

impl From<AuthFromRequestError> for HandlerError {
    fn from(error: AuthFromRequestError) -> Self {
        match error {
            AuthFromRequestError::DbPoolError(e) => HandlerError::PoolError(e),
            AuthFromRequestError::UserIdDecodeError(e) => HandlerError::ParseIntError(e),
            AuthFromRequestError::DbQueryError(e) => HandlerError::DatabaseError(e),
        }
    }
}

impl From<GithubClientError> for HandlerError {
    fn from(error: GithubClientError) -> Self {
        match error {
            GithubClientError::HttpError(e) => HandlerError::HttpError(e),
        }
    }
}
