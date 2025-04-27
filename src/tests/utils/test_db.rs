use crate::db::pool::DbPool;
use diesel::r2d2::ConnectionManager;
use diesel::{Connection, PgConnection, RunQueryDsl};
use r2d2::Pool;
use std::env;
use uuid::Uuid;

pub struct TestDatabase {
    pub database_name: String,
}

impl TestDatabase {
    pub fn new() -> Self {
        let base_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // Connect without specifying database
        let base_url_parts: Vec<&str> = base_url.rsplitn(2, '/').collect();
        let server_url = base_url_parts[1];

        let mut admin_connection = PgConnection::establish(&format!("{}/postgres", server_url))
            .expect("Failed to connect to Postgres");

        let database_name = format!("test_db_{}", Uuid::new_v4().to_string());

        diesel::sql_query(format!("CREATE DATABASE {}", database_name))
            .execute(&mut admin_connection)
            .expect("Failed to create test database");

        Self { database_name }
    }

    pub fn create_pool(&self) -> DbPool {
        let base_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let base_url_parts: Vec<&str> = base_url.rsplitn(2, '/').collect();
        let server_url = base_url_parts[1];

        let db_url = format!("{}/{}", server_url, self.database_name);

        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        let base_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let base_url_parts: Vec<&str> = base_url.rsplitn(2, '/').collect();
        let server_url = base_url_parts[1];

        let mut admin_connection = PgConnection::establish(&format!("{}/postgres", server_url))
            .expect("Failed to connect to Postgres for cleanup");

        // Terminate connections
        let terminate_connections = format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';",
            self.database_name
        );
        diesel::sql_query(terminate_connections)
            .execute(&mut admin_connection)
            .expect("Failed to terminate connections");

        // Drop database
        diesel::sql_query(format!("DROP DATABASE {}", self.database_name))
            .execute(&mut admin_connection)
            .expect("Failed to drop test database");
    }
}
