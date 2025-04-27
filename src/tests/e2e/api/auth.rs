#[cfg(test)]
mod tests {
    use crate::api::dto::auth::AuthResponse;
    use crate::api::dto::generic::HttpResponse;
    use crate::tests::utils::helpers::parse_response;
    use crate::tests::utils::setup::rocket_with_db;
    use rocket::http::{ContentType, Status};

    #[tokio::test]
    async fn test_register_success() {
        let client = rocket_with_db().await;

        let response = client
            .post("/auth/register")
            .header(ContentType::JSON)
            .body(r#"{ "username": "newuser", "password": "password123" }"#)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let parsed: HttpResponse<AuthResponse> = parse_response(response).await;

        match parsed {
            HttpResponse::Success(data) => {
                assert!(!data.token.is_empty(), "Token should not be empty");
            }
            _ => panic!("Expected success response"),
        }
    }

    #[tokio::test]
    async fn test_register_conflict() {
        let client = rocket_with_db().await;

        // First registration
        client
            .post("/auth/register")
            .header(ContentType::JSON)
            .body(r#"{ "username": "existinguser", "password": "password123" }"#)
            .dispatch()
            .await;

        // Second registration attempt
        let response = client
            .post("/auth/register")
            .header(ContentType::JSON)
            .body(r#"{ "username": "existinguser", "password": "password123" }"#)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Conflict);

        let parsed: HttpResponse<AuthResponse> = parse_response(response).await;

        match parsed {
            HttpResponse::Error(err) => {
                assert_eq!(err.message, "User already registered");
            }
            _ => panic!("Expected conflict error response"),
        }
    }

    #[tokio::test]
    async fn test_login_success() {
        let client = rocket_with_db().await;

        // Register first
        client
            .post("/auth/register")
            .header(ContentType::JSON)
            .body(r#"{ "username": "loginuser", "password": "password123" }"#)
            .dispatch()
            .await;

        // Then login
        let response = client
            .post("/auth/login")
            .header(ContentType::JSON)
            .body(r#"{ "username": "loginuser", "password": "password123" }"#)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let parsed: HttpResponse<AuthResponse> = parse_response(response).await;

        match parsed {
            HttpResponse::Success(data) => {
                assert!(!data.token.is_empty(), "Token should not be empty");
            }
            _ => panic!("Expected successful login response"),
        }
    }

    #[tokio::test]
    async fn test_login_invalid_password() {
        let client = rocket_with_db().await;

        // Register first
        client
            .post("/auth/register")
            .header(ContentType::JSON)
            .body(r#"{ "username": "wrongpassuser", "password": "password123" }"#)
            .dispatch()
            .await;

        // Try login with wrong password
        let response = client
            .post("/auth/login")
            .header(ContentType::JSON)
            .body(r#"{ "username": "wrongpassuser", "password": "wrongpass" }"#)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Unauthorized);

        let parsed: HttpResponse<AuthResponse> = parse_response(response).await;

        match parsed {
            HttpResponse::Error(err) => {
                assert_eq!(err.message, "Invalid password");
            }
            _ => panic!("Expected invalid password error"),
        }
    }

    #[tokio::test]
    async fn test_login_user_not_found() {
        let client = rocket_with_db().await;

        let response = client
            .post("/auth/login")
            .header(ContentType::JSON)
            .body(r#"{ "username": "nonexistent", "password": "whatever" }"#)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::NotFound);

        let parsed: HttpResponse<AuthResponse> = parse_response(response).await;

        match parsed {
            HttpResponse::Error(err) => {
                assert_eq!(err.message, "User not found");
            }
            _ => panic!("Expected user not found error"),
        }
    }
}
