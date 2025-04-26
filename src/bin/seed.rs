use diesel::prelude::*;
use diesel::pg::PgConnection;
use uuid::Uuid;
use bookstore::db;
use bookstore::db::models::user::User;

fn main() {
    let pool = db::pool::create_database_pool();
    let mut conn = pool.get().expect("Failed to get db connection");
    
    seed_users(&mut conn)
}

pub fn seed_users(conn: &mut PgConnection) {
    use bookstore::db::schema::users::dsl::*;

    // Check if users exist
    let count: i64 = users
        .count()
        .get_result(conn)
        .expect("Failed to count users");

    if count > 0 {
        println!("✅ Users already seeded. Skipping...");
        return;
    }

    println!("🌱 Seeding users...");

    let new_users = vec![
        User {
            id: Uuid::new_v4(),
            username: "Alice".into(),
            password: "<PASSWORD>".into(),
        },
        User {
            id: Uuid::new_v4(),
            username: "Bob".into(),
            password: "<PASSWORD>".into(),
        },
        User {
            id: Uuid::new_v4(),
            username: "Charlie".into(),
            password: "<PASSWORD>".into(),
        },
    ];

    diesel::insert_into(users)
        .values(&new_users)
        .execute(conn)
        .expect("Failed to seed users");

    println!("✅ Seeded {} users", new_users.len());
}