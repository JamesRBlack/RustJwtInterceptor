use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use crate::types::Claims;
use dotenvy::dotenv;
use uuid::Uuid;

pub fn generate_jwt() -> String {
    dotenv().ok(); // Load .env if it exists

    let now = Utc::now();
    let jti = Uuid::new_v4().to_string(); // Generate random UUID
    let claims = Claims {
        sub: "continue-proxy-client".to_string(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::minutes(5)).timestamp() as usize,
        iss: "rust-proxy".to_string(),
        jti, 
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(
            std::env::var("JWT_SECRET").expect("JWT_SECRET missing").as_bytes()
        ),
    ).unwrap()
}
