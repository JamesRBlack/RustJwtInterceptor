use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use crate::types::Claims;

const SHARED_SECRET: &str = "your_shared_secret_key";

pub fn generate_jwt() -> String {
    let now = Utc::now();
    let claims = Claims {
        sub: "continue-proxy-client".to_string(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::minutes(5)).timestamp() as usize,
        iss: "rust-proxy".to_string(),
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SHARED_SECRET.as_bytes()),
    ).unwrap()
}
