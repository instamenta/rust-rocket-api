use rocket::serde::{Deserialize, DeserializeOwned, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HttpErrorMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub enum HttpResponse<T> {
    Success(T),
    Error(HttpErrorMessage),
}

impl<T> HttpResponse<T> {
    pub(crate) fn error(message: String) -> Self {
        HttpResponse::Error(HttpErrorMessage { message })
    }

    pub fn success(data: T) -> Self {
        HttpResponse::Success(data)
    }
}
