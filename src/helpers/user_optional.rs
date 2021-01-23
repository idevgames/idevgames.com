use crate::{
    db::DbConn,
    helpers::TemplateContextUser,
    models::{GithubUserRecord, User},
};
use actix_session::Session;
use serde::Serialize;

use super::{auth_from_session, AuthFromSessionError};

pub struct UserOptional {
    /// The current user, or is it?
    user: Option<(User, GithubUserRecord)>,

    /// The permissions the current user has, if any.
    permissions: Vec<String>,
}

/// This is the context that goes to the template itself. To check for the
/// presence of a user, use the `is object` test. This should always be in the
/// `auth` field of a template context.
#[derive(Debug, Serialize)]
pub struct UserOptionalContext {
    /// The user, or is it?
    user: Option<TemplateContextUser>,
}

impl UserOptional {
    pub fn from_session(
        conn: &DbConn,
        session: &Session,
    ) -> Result<UserOptional, AuthFromSessionError> {
        match auth_from_session(conn, session) {
            Ok(Some((user, permissions))) => {
                let github_user = GithubUserRecord::find_by_user_id(conn, user.id)?;

                match github_user {
                    Some(github_user) => Ok(UserOptional {
                        user: Some((user, github_user)),
                        permissions,
                    }),
                    None => Err(AuthFromSessionError::GithubUserRecordNotFound(user.id)),
                }
            }
            Ok(None) => Ok(UserOptional {
                user: None,
                permissions: vec![],
            }),
            Err(e) => Err(e),
        }
    }

    /// Produces a serializable context that can be passed to a template.
    pub fn to_context(&self) -> UserOptionalContext {
        return UserOptionalContext {
            user: match &self.user {
                Some(u) => Some(TemplateContextUser {
                    id: u.0.id,
                    login: u.1.login.clone(),
                    permissions: self.permissions.clone(),
                }),
                None => None,
            },
        };
    }

    pub fn is_admin(&self) -> bool {
        self.permissions.contains(&"admin".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tera::{Context, Tera};

    /// Validates the detection of a logged in user in a template. If this
    /// breaks (highly unlikely) then a number of templates also need to be
    /// updated.
    #[test]
    fn test_user_optional_template_context() {
        let none_context = UserOptionalContext { user: None };
        let some_context = UserOptionalContext {
            user: Some(TemplateContextUser {
                id: 1,
                login: "ed".to_string(),
                permissions: vec!["admin".to_string()],
            }),
        };
        let mut tera = Tera::default();
        tera.add_raw_template(
            "example.html",
            "
            {% if user is object %}
            The user is logged in!
            {% else %}
            There is no user logged in.
            {% endif %}
        ",
        )
        .unwrap();
        let none_result = tera
            .render(
                "example.html",
                &Context::from_serialize(&none_context).unwrap(),
            )
            .unwrap();
        let some_result = tera
            .render(
                "example.html",
                &Context::from_serialize(&some_context).unwrap(),
            )
            .unwrap();
        assert_eq!("There is no user logged in.", none_result.trim());
        assert_eq!("The user is logged in!", some_result.trim());
    }
}
