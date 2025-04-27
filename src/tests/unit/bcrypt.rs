#[cfg(test)]
mod tests {
    use crate::utils::bcrypt::BCrypt;

    #[test]
    fn test_hash_and_verify_password_success() {
        let password = "my_secure_password";
        let hashed = BCrypt::hash_password(password);

        assert!(
            BCrypt::verify_password(password, &hashed),
            "Password should verify correctly"
        );
    }

    #[test]
    fn test_verify_password_failure() {
        let password = "my_secure_password";
        let wrong_password = "wrong_password";
        let hashed = BCrypt::hash_password(password);

        assert!(
            !BCrypt::verify_password(wrong_password, &hashed),
            "Wrong password should not verify"
        );
    }

    #[test]
    fn test_hash_password_different_each_time() {
        let password = "same_password";
        let hash1 = BCrypt::hash_password(password);
        let hash2 = BCrypt::hash_password(password);

        assert_ne!(
            hash1, hash2,
            "Hashing the same password should produce different hashes due to salt"
        );
    }
}