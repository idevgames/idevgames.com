use crate::{
    db::DbConn,
    models::{r_to_opt, ModelError},
};

use super::users::User;

/// Local cache of part of Github's understanding of who a user is. Particularly
/// the id, which persists across use renames, and the user's login, which is a
/// human-readable name for the user.
#[derive(Debug, Queryable)]
pub struct GithubUserRecord {
    /// A unique id for this user, supplied by Github and used here as a primary
    /// key.
    pub id: i64,

    /// The id of the iDevGames-side user id.
    pub user_id: i32,

    /// The user's human-readable name.
    pub login: String,

    /// Url of this user's picture.
    pub avatar_url: String,

    /// Url of this user's Github profile.
    pub html_url: String,
}

impl GithubUserRecord {
    /// Finds or creates a GhUserRecord in the database with the given gh_id,
    /// and ensures that it has the given attributes.
    pub fn find_and_update(
        conn: &DbConn,
        the_id: i64,
        the_user_id: i32,
        the_login: &str,
        the_avatar_url: &str,
        the_html_url: &str,
    ) -> Result<Self, ModelError> {
        use crate::schema::github_user_records::dsl::{
            avatar_url, github_user_records, html_url, id, login, user_id,
        };
        use diesel::prelude::*;

        match Self::find_by_id(conn, the_id)? {
            Some(u) => {
                if the_login != u.login
                    || the_avatar_url != u.avatar_url
                    || the_html_url != u.html_url
                {
                    diesel::update(github_user_records.find(the_id))
                        .set((
                            user_id.eq(the_user_id),
                            login.eq(the_login),
                            avatar_url.eq(the_avatar_url),
                            html_url.eq(the_html_url),
                        ))
                        .execute(conn)?;
                }
            }
            None => {
                diesel::insert_into(github_user_records)
                    .values((
                        id.eq(the_id),
                        user_id.eq(the_user_id),
                        login.eq(the_login),
                        avatar_url.eq(the_avatar_url),
                        html_url.eq(the_html_url),
                    ))
                    .execute(conn)?;
            }
        };

        Ok(Self::find_by_id(&conn, the_id)?.unwrap())
    }

    /// Finds a given GhUserRecord by its id.
    pub fn find_by_id(conn: &DbConn, the_id: i64) -> Result<Option<Self>, ModelError> {
        use crate::schema::github_user_records::dsl::{github_user_records, id};
        use diesel::prelude::*;

        let user_record = github_user_records
            .filter(id.eq(the_id))
            .limit(1)
            .first::<Self>(conn);

        r_to_opt(user_record)
    }

    /// Finds a given GhUserRecord by its login.
    pub fn find_by_login(conn: &DbConn, the_login: &str) -> Result<Option<Self>, ModelError> {
        use crate::schema::github_user_records::dsl::{github_user_records, login};
        use diesel::prelude::*;

        let user_record = github_user_records
            .filter(login.eq(the_login))
            .limit(1)
            .first::<Self>(conn);

        r_to_opt(user_record)
    }

    /// Finds a given GithubUserRecord by its local iDevGames-side user id.
    pub fn find_by_user_id(conn: &DbConn, the_user_id: i32) -> Result<Option<Self>, ModelError> {
        use crate::schema::github_user_records::dsl::{github_user_records, user_id};
        use diesel::prelude::*;

        let user_record = github_user_records
            .filter(user_id.eq(the_user_id))
            .limit(1)
            .first::<Self>(conn);

        r_to_opt(user_record)
    }

    /// Gets the User this GithubUserRecord corresponds to.
    pub fn get_user(&self, conn: &DbConn) -> Result<User, ModelError> {
        let u = User::find_by_id(&conn, self.user_id).transpose();

        if u.is_none() {
            Err(ModelError::NotFound)
        } else {
            u.unwrap()
        }
    }
}
