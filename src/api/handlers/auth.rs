use rocket::*;
use rocket::http;
use rocket::serde::json::Json;
use crate::api::dto;
use crate::api::dto::auth::AuthResponse;
use crate::api::dto::generic::{HttpResponse};
use crate::db::repositories::user::{IUserRepository};
use crate::utils::bcrypt::BCrypt;
use crate::utils::jwt::JWT;

#[post("/login", data = "<auth_data>")]
pub fn login(
    auth_data: Json<dto::auth::AuthRequest>,
    user_repository: &State<Box<dyn IUserRepository>>,
    jwt: &State<JWT>,
) -> (http::Status, Json<HttpResponse<AuthResponse>>) {
    // TODO: validate data

    match user_repository.get_user_by_username(&auth_data.username) {
        Some(user) => {
            let is_password_valid = BCrypt::verify_password(&auth_data.password, &user.password);
            if !is_password_valid {
                return (
                    http::Status::Unauthorized,
                    Json(HttpResponse::error("Invalid password".to_string())),
                );
            }

            (
                http::Status::Ok,
                Json(HttpResponse::success(AuthResponse { token: jwt.create_token(&user.username) })),
            )
        }
        None => (
            http::Status::NotFound,
            Json(HttpResponse::error("User not found".to_string())),
        ),
    }
}

#[post("/register", data = "<auth_data>")]
pub fn register(
    auth_data: Json<dto::auth::AuthRequest>,
    user_repository: &State<Box<dyn IUserRepository>>,
    jwt: &State<JWT>,
) -> (http::Status, Json<HttpResponse<AuthResponse>>) {
    // TODO: validate username and password

    if let Some(_) = user_repository.get_user_by_username(&auth_data.username) {
        return (
            http::Status::Conflict,
            Json(HttpResponse::error("User already registered".to_string() )),
        );
    }

    let hashed_password = BCrypt::hash_password(&auth_data.password);

    let new_user = user_repository.create_user(auth_data.username.clone(), hashed_password);

    let response = AuthResponse { token: jwt.create_token(new_user.username.as_str()) };

    (http::Status::Ok, Json(HttpResponse::success(response)))
}
