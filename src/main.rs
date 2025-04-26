#[macro_use] extern crate rocket;

use dotenvy::dotenv;
use bookstore::db;
use bookstore::api;
use bookstore::db::repositories::user::IUserRepository;
use bookstore::utils::jwt::JWT;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let pool = db::pool::create_database_pool();

    db::migration::run_migrations(pool.clone());

    const SECRET: &[u8] = b"secret_key";

    let jwt = JWT::new(SECRET);
    let user_repository = db::repositories::user::UserRepository::new(pool);

    rocket::build()

        .manage(jwt)
        .manage(Box::new(user_repository) as Box<dyn IUserRepository>)

        .mount("/auth", routes![
            api::handlers::auth::register,
            api::handlers::auth::login,
        ])
        .mount("/", routes![index])
}
