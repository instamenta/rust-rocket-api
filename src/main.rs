#[macro_use] extern crate rocket;

use bookstore::db;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let pool = db::pool::create_database_pool();

    let user_list = db::repositories::user::load_users(pool);

    println!("Loaded Users {:?}", user_list);

    rocket::build().mount("/", routes![index])
}
