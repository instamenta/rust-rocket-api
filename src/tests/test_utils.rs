use rocket::local::asynchronous::Client;
use rocket::routes;
use crate::tests::mocks::mock_user_repository::MockUserRepository;
use crate::utils::jwt::JWT;
use crate::api;
use crate::db::repositories::user::IUserRepository;

pub async fn setup_rocket() -> Client {
    let user_repository: Box<dyn IUserRepository> = Box::new(MockUserRepository {});
    let jwt = JWT::new(b"test_secret");

    let rocket = rocket::build()
        
        .manage(user_repository)
        .manage(jwt)
        
        .mount("/auth", routes![
            api::handlers::auth::register,
            api::handlers::auth::login
        ]);

    Client::tracked(rocket).await.expect("valid rocket instance")
}