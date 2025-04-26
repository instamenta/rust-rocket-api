use diesel::prelude::*;
use crate::db::models::user::User;
use crate::db::pool::DbPool;
use crate::db::schema::users::dsl::users;

pub fn load_users(pool: DbPool) -> Vec<User> {
    let mut conn = pool.get().expect("Failed to get db connection");

    users.load::<User>(&mut conn).expect("Failed to load users")
}