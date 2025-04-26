use diesel::prelude::*;
use uuid::Uuid;
use crate::db::models::user::User;
use crate::db::pool::DbPool;
use crate::db::schema::users::dsl::users;

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

    pub fn load_users(&self) -> Vec<User> {
        let mut conn = self.pool.get().expect("Failed to get db connection");

        users.load::<User>(&mut conn).expect("Failed to load users")
    }

    pub fn create_user(&self, username: String, password: String) -> User {
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

    pub fn get_user_by_username(&self, search_username: &str) -> Option<User> {
        let mut conn = self.pool.get().expect("Failed to get db connection");

        users
            .filter(crate::db::schema::users::dsl::username.eq(search_username))
            .first::<User>(&mut conn)
            .ok()
    }
}