use crate::db::models::user::User;
use crate::db::pool::DbPool;
use crate::db::schema::users::dsl::users;
use diesel::prelude::*;
use uuid::Uuid;

pub trait IUserRepository: Send + Sync {
    fn get_user_by_username(&self, username: &str) -> Option<User>;
    fn create_user(&self, username: String, password: String) -> User;
}

#[derive(Clone)]
pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl IUserRepository for UserRepository {
    fn get_user_by_username(&self, search_username: &str) -> Option<User> {
        let mut conn = self.pool.get().expect("Failed to get db connection");

        users
            .filter(crate::db::schema::users::dsl::username.eq(search_username))
            .first::<User>(&mut conn)
            .ok()
    }

    fn create_user(&self, username: String, password: String) -> User {
        let mut conn = self.pool.get().expect("Failed to get db connection");

        let new_user = User {
            id: Uuid::new_v4(),
            username,
            password,
        };

        diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(&mut conn)
            .unwrap()
    }
}
