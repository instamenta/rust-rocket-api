use rocket::*;
use rocket::http::Status;
use rocket::serde::json;
use crate::api::dto;
use crate::db::pool::DbPool;
use crate::db::repositories;

#[post("/login", data = "<auth_data>")]
pub fn login(
    auth_data: json::Json<dto::auth::AuthRequest>,
    pool: &State<DbPool>
) -> (Status, json::Json<dto::auth::AuthResponse>) {
    repositories::user::get_user_by_username(pool.inner(), &auth_data.username);
}

#[post("/register", data = "<auth_data>")]
pub fn register(
    auth_data: json::Json<dto::auth::AuthRequest>,
    pool: &State<DbPool>
) -> (Status, json::Json<dto::auth::AuthResponse>) {
    auth_data;
    todo!();
}
