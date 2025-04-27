use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::db::models::user::User;
use crate::db::repositories::user::IUserRepository;

pub struct MockUserRepository {
    users: Arc<Mutex<HashMap<String, User>>>,
}

impl MockUserRepository {
    pub fn new() -> Self { Self { users: Arc::new(Mutex::new(HashMap::new())) } }
}


impl IUserRepository for MockUserRepository {
    fn get_user_by_username(&self, username: &str) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.get(username).cloned()
    }

    fn create_user(&self, username: String, password: String) -> User {
        let mut users = self.users.lock().unwrap();

        let user = User {
            id: Uuid::new_v4(),
            username: username.clone(),
            password,
        };

        users.insert(username, user.clone());

        user
    }
}