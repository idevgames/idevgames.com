//! Models put an abstraction between controller or "business logic" code and
//! the database itself. They hide SQL queries and include logic which is
//! intended to keep the database consistent. You should never manipulate the
//! database directly from either command-line tool or controller code.

pub(crate) mod github_user_records;
pub(crate) mod permissions;
pub(crate) mod snippets;
pub(crate) mod users;

use diesel::{r2d2::PoolError, result::Error as DieselError};
use thiserror::Error;

pub use github_user_records::GithubUserRecord;
pub use permissions::Permission;
pub use snippets::Snippet;
pub use users::User;

/// An error common to model helper functions.
#[derive(Error, Debug)]
pub enum ModelError {
    /// Failed to get a database connection.
    #[error("Couldn't get out of the pool with error {0}. Send a lifeguard.")]
    PoolError(#[from] PoolError),

    /// Failed to query the database, or no result from the database when one
    /// was expected.
    #[error("Couldn't query the database with error {0}. Send a DBA.")]
    DieselError(#[from] DieselError),

    /// The entity was not found in the database. This indicates that the lack
    /// of an associated record signifies a schema violation.
    #[error("The entity was not found.")]
    NotFound,
}

// Gets the most recently inserted row. Please only use this from within a
// transaction to avoid threading adventures.
no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

/// Converts a diesel result, which packages the absence of a record as an
/// error, into an Option, which makes dealing with "I'm okay with something not
/// being present" slightly more Rustic.
fn r_to_opt<T>(r: Result<T, diesel::result::Error>) -> Result<Option<T>, ModelError> {
    match r {
        Ok(t) => Ok(Some(t)),
        Err(diesel::NotFound) => Ok(None),
        Err(e) => Err(ModelError::DieselError(e)),
    }
}
