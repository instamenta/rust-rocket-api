use rocket::local::asynchronous::Client;
use rocket::routes;
use crate::{api, db};
use crate::db::repositories::user::{IUserRepository, UserRepository};
use crate::tests::mocks::mock_user_repository::MockUserRepository;
use crate::tests::utils::test_db::TestDatabase;
use crate::utils::jwt::JWT;

pub async fn rocket_with_mock_db() -> Client {
    let user_repository: Box<dyn IUserRepository> = Box::new(MockUserRepository::new());
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

pub async fn rocket_with_db() -> Client {
    dotenvy::dotenv().ok();

    let test_db = TestDatabase::new();
    let pool = test_db.create_pool();

    db::migration::run_migrations(pool.clone());

    let jwt = JWT::new(b"secret_key");
    let user_repository = UserRepository::new(pool);

    let rocket = rocket::build()
        .manage(jwt)
        .manage(Box::new(user_repository) as Box<dyn IUserRepository>)

        .mount("/auth", routes![
            api::handlers::auth::register,
            api::handlers::auth::login
        ]);

    Client::tracked(rocket).await.expect("valid rocket instance")
}