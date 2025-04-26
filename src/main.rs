#[macro_use] extern crate rocket;

use dotenvy::dotenv;
use bookstore::db;
use bookstore::api;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let pool = db::pool::create_database_pool();

    rocket::build()
        .manage(&pool)
        .mount("/auth", routes![
            api::handlers::auth::register,
            api::handlers::auth::login,
        ])
        .mount("/", routes![index])
}
