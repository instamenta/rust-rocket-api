use crate::db::pool::DbPool;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(pool: DbPool) {
    let mut conn = pool
        .get()
        .expect("Failed to get db connection for migrations");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run database migrations");
}
