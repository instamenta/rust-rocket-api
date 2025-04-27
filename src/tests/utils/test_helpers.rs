use serde_json::from_str;

pub async fn parse_response<T: for<'de> serde::Deserialize<'de>>(
    response: rocket::local::asynchronous::LocalResponse<'_>,
) -> T {
    let body = response.into_string().await.expect("response body");
    from_str::<T>(&body).expect("valid JSON")
}
