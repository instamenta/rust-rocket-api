use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct HttpErrorMessage {
    pub message: String,
}

#[derive(Serialize)]
#[serde(bound = "T: Serialize")]
pub enum HttpResponse <T> {
    Success(T),
    Error(HttpErrorMessage),
}

impl <T> HttpResponse<T> {
    pub(crate) fn error(message: String) -> Self {
        HttpResponse::Error(HttpErrorMessage { message })
    }

    pub fn success(data: T) -> Self {
        HttpResponse::Success(data)
    }
}

