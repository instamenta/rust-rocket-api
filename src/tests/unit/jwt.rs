
#[cfg(test)]
mod tests {
    use crate::utils::jwt::JWT;
    use chrono::Utc;

    const SECRET: &[u8] = b"secret_key_mock";

    #[test]
    fn test_create_and_verify_token_success() {
        let jwt = JWT::new(SECRET);

        let username = "testuser";
        let token = jwt.create_token(username);

        let claims = jwt.verify_token(&token).expect("Token should be valid");

        assert_eq!(claims.sub, username);
        assert!(claims.exp > Utc::now().timestamp() as usize);
    }

    #[test]
    fn test_verify_token_invalid_secret() {
        let jwt_good = JWT::new(SECRET);
        let jwt_bad = JWT::new(b"wrong_secret");

        let username = "testuser";
        let token = jwt_good.create_token(username);

        let claims = jwt_bad.verify_token(&token);

        assert!(claims.is_none(), "Verification should fail with wrong secret");
    }

    #[test]
    fn test_verify_invalid_token_format() {
        let jwt = JWT::new(SECRET);

        let invalid_token = "this.is.not.a.valid.token";

        let claims = jwt.verify_token(invalid_token);

        assert!(claims.is_none(), "Should fail for completely invalid token format");
    }
}