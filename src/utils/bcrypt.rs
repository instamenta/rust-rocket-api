use bcrypt::{hash, verify};

pub struct BCrypt {}

impl BCrypt {
    pub fn hash_password(password: &str) -> String {
        hash(password, 4).expect("Password hashing failed")
    }

    pub fn verify_password(password: &str, hashed: &str) -> bool {
        verify(password, hashed).unwrap_or(false)
    }
}
