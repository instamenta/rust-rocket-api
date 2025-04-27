use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct JWT {
    secret: &'static [u8],
}

impl JWT {
    pub fn new(secret: &'static [u8]) -> Self {
        JWT { secret }
    }

    pub fn create_token(&self, username: &str) -> String {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: username.to_owned(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret),
        )
        .unwrap()
    }

    pub fn verify_token(&self, token: &str) -> Option<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .ok()
    }
}
