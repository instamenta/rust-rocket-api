use uuid::Uuid;
use crate::db::models::user::User;
use crate::db::repositories::user::IUserRepository;

pub struct MockUserRepository;

impl IUserRepository for MockUserRepository {
    fn get_user_by_username(&self, username: &str) -> Option<User> {
        if username == "existing_user" {
            Some(User {
                id: Uuid::new_v4(),
                username: username.to_string(),
                password: "hashed_password".to_string(),
            })
        } else {
            None
        }
    }

    fn create_user(&self, username: String, password: String) -> User {
        User {
            id: Uuid::new_v4(),
            username,
            password,
        }
    }
}