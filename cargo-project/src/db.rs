use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    SqliteConnection,
};
use diesel_migrations::embed_migrations;
use parking_lot::RwLock;
use std::sync::Arc;

embed_migrations!("../migrations");

pub type DbManager = ConnectionManager<SqliteConnection>;
pub type DbPool = Arc<RwLock<Pool<DbManager>>>;
pub type DbConn = PooledConnection<ConnectionManager<SqliteConnection>>;

// gets the pool.
pub fn get_pool(db_path: &str, maxconns: u32) -> DbPool {
    let conn_manager: ConnectionManager<SqliteConnection> = ConnectionManager::new(db_path);
    let pool = Pool::builder()
        .max_size(maxconns)
        .build(conn_manager)
        .unwrap();

    Arc::new(RwLock::new(pool))
}

/// migrates the db. panics if there is a migration failure.
pub fn migrate_db(pool: &DbPool) {
    let conn = pool.read().get().unwrap();
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();
}
