use clap::Clap;

use crate::{application_context::ApplicationContext, db::migrate_db};

/// Migrates the iDevGames database to the current schema
#[derive(Clap, Debug)]
pub struct Migrate {}

impl Migrate {
    pub fn migrate(&self, ctxt: &ApplicationContext) {
        println!("Migrating the database!");
        migrate_db(&ctxt.db_pool);
    }
}
