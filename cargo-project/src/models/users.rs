use crate::{
    db::DbConn,
    models::{last_insert_rowid, r_to_opt, ModelError},
};
use diesel::result::Error as DieselError;

/// The iDevGames-side structure describing what a user is. A User may "have"
/// one or more other kinds of persona, such as a Github record if that user
/// logged in using Github.
#[derive(Debug, Queryable)]
pub struct User {
    /// A unique id for this user.
    pub id: i32,

    /// The preferred name of this user. Currently totally unused, but required
    /// because inserting things with no values makes for an incomplete insert
    /// statement.
    pub preferred_name: String,
}

impl User {
    pub fn create(conn: &DbConn) -> Result<User, ModelError> {
        use crate::schema::users::dsl::{id, preferred_name, users};
        use diesel::prelude::*;

        let u = conn.transaction::<User, DieselError, _>(|| {
            diesel::insert_into(users)
                // it's a good name.
                .values(preferred_name.eq("Bob"))
                .execute(conn)?;
            let rowid = diesel::select(last_insert_rowid).get_result::<i32>(conn)?;
            Ok(users.filter(id.eq(rowid)).limit(1).first::<Self>(conn)?)
        })?;

        Ok(u)
    }

    pub fn find_by_id(conn: &DbConn, the_id: i32) -> Result<Option<User>, ModelError> {
        use crate::schema::users::dsl::{id, users};
        use diesel::prelude::*;

        let u = users.filter(id.eq(the_id)).limit(1).first::<Self>(conn);

        r_to_opt(u)
    }
}
