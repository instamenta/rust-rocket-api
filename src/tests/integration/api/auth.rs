#[cfg(test)]
mod tests {
    use crate::api::dto::auth::AuthResponse;
    use crate::api::dto::generic::HttpResponse;
    use crate::tests::utils::helpers::parse_response;
    use crate::tests::utils::setup;
    use rocket::http::{ContentType, Status};
    use rocket::tokio;

    #[tokio::test]
    async fn test_register_success() {
        let client = setup::rocket_with_mock_db().await;

        let response = client
            .post("/auth/register")
            .header(ContentType::JSON)
            .body(r#"{ "username": "testuser", "password": "password123" }"#)
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
        let client = setup::rocket_with_mock_db().await;

        // First register
        client
            .post("/auth/register")
            .header(ContentType::JSON)
            .body(r#"{ "username": "existing_user", "password": "password123" }"#)
            .dispatch()
            .await;

        // Try to register again
        let response = client
            .post("/auth/register")
            .header(ContentType::JSON)
            .body(r#"{ "username": "existing_user", "password": "password123" }"#)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Conflict);

        let parsed: HttpResponse<AuthResponse> = parse_response(response).await;

        match parsed {
            HttpResponse::Error(err) => {
                assert_eq!(err.message, "User already registered");
            }
            _ => panic!("Expected error response"),
        }
    }

    #[tokio::test]
    async fn test_login_success() {
        let client = setup::rocket_with_mock_db().await;

        // Register the user first
        client
            .post("/auth/register")
            .header(ContentType::JSON)
            .body(r#"{ "username": "login_user", "password": "password123" }"#)
            .dispatch()
            .await;

        // Then login
        let response = client
            .post("/auth/login")
            .header(ContentType::JSON)
            .body(r#"{ "username": "login_user", "password": "password123" }"#)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let parsed: HttpResponse<AuthResponse> = parse_response(response).await;

        match parsed {
            HttpResponse::Success(data) => {
                assert!(!data.token.is_empty(), "Token should not be empty");
            }
            _ => panic!("Expected success login response"),
        }
    }

    #[tokio::test]
    async fn test_login_invalid_password() {
        let client = setup::rocket_with_mock_db().await;

        // Register the user first
        client
            .post("/auth/register")
            .header(ContentType::JSON)
            .body(r#"{ "username": "wrong_pass_user", "password": "password123" }"#)
            .dispatch()
            .await;

        // Try to login with wrong password
        let response = client
            .post("/auth/login")
            .header(ContentType::JSON)
            .body(r#"{ "username": "wrong_pass_user", "password": "wrongpassword" }"#)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Unauthorized);

        let parsed: HttpResponse<AuthResponse> = parse_response(response).await;

        match parsed {
            HttpResponse::Error(err) => {
                assert_eq!(err.message, "Invalid password");
            }
            _ => panic!("Expected error response for invalid password"),
        }
    }

    #[tokio::test]
    async fn test_login_user_not_found() {
        let client = setup::rocket_with_mock_db().await;

        // Try to login without registering
        let response = client
            .post("/auth/login")
            .header(ContentType::JSON)
            .body(r#"{ "username": "nonexistent_user", "password": "whatever" }"#)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::NotFound);

        let parsed: HttpResponse<AuthResponse> = parse_response(response).await;

        match parsed {
            HttpResponse::Error(err) => {
                assert_eq!(err.message, "User not found");
            }
            _ => panic!("Expected error response for user not found"),
        }
    }
}
