#[cfg(test)]
mod tests {
    use rocket::http::{Status, ContentType};
    use crate::tests::test_utils;
    use rocket::local::asynchronous::Client; // important for async
    use rocket::tokio; // for async tests

    #[tokio::test]
    async fn test_register_success() {
        let client = test_utils::setup_rocket().await;

        let response = client.post("/auth/register")
            .header(ContentType::JSON)
            .body(r#"{ "username": "testuser", "password": "password123" }"#)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let body = response.into_string().await.unwrap();
        println!("BODY: {}", body);

        assert!(body.contains("Success"));
        assert!(body.contains("token"));
    }

    #[tokio::test]
    async fn test_register_conflict() {
        let client = test_utils::setup_rocket().await;

        let response = client.post("/auth/register")
            .header(ContentType::JSON)
            .body(r#"{ "username": "existing_user", "password": "password123" }"#)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Conflict);

        let body = response.into_string().await.unwrap();
        println!("BODY: {}", body);

        assert!(body.contains("Error"));
        assert!(body.contains("User already registered"));
    }
}
