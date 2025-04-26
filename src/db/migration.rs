use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::db::pool::DbPool;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(pool: DbPool) {
    let mut conn = pool.get().expect("Failed to get db connection for migrations");
    conn.run_pending_migrations(MIGRATIONS).expect("Failed to run database migrations");
}