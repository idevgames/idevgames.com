use crate::{
    db::DbConn,
    helpers::TemplateContextUser,
    models::{GithubUserRecord, User},
};
use actix_session::Session;
use serde::Serialize;

use super::{auth_from_session, AuthFromSessionError};

pub struct AdminOnly {
    user: (User, GithubUserRecord),
    permissions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminOnlyContext {
    user: TemplateContextUser,
}

impl AdminOnly {
    pub fn from_session(
        conn: &DbConn,
        session: &Session,
    ) -> Result<AdminOnly, AuthFromSessionError> {
        match auth_from_session(conn, session) {
            Ok(Some((user, permissions))) => {
                if !permissions.contains(&"admin".to_owned()) {
                    return Err(AuthFromSessionError::RoleNotMatched(
                        "admin".to_owned(),
                        permissions,
                    ));
                }

                let github_user = GithubUserRecord::find_by_user_id(conn, user.id)?;

                match github_user {
                    Some(github_user) => Ok(AdminOnly {
                        user: (user, github_user),
                        permissions,
                    }),
                    None => Err(AuthFromSessionError::GithubUserRecordNotFound(user.id)),
                }
            }
            Ok(None) => Err(AuthFromSessionError::NoUser),
            Err(e) => Err(e),
        }
    }

    pub fn to_context(&self) -> AdminOnlyContext {
        return AdminOnlyContext {
            user: TemplateContextUser {
                id: self.user.0.id,
                login: self.user.1.login.clone(),
                permissions: self.permissions.clone(),
            },
        };
    }
}
