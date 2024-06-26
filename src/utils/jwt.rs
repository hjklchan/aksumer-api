use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::error::api_error::{ApiError, AuthenticateError};

/// A KEYS (Encoding/DecodingKey) which is initialized on the first access
static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("AUTH_SECRET").expect("AUTH_SECRET must be set");
    Keys::new(secret.as_bytes())
});

static EXPIRE: Lazy<i64> = Lazy::new(|| {
    let secs = std::env::var("AUTH_EXPIRE").expect("invalid AUTH_EXPIRE");
    secs.parse().expect("Failed to parse to u64 type")
});

/// ## Keys contains (Encoding/Decoding)Key
///
/// Handling secret from &\[u8\] type
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// ## JWT payload
#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub id: u64,
    pub username: String,
}

/// JWT Claims
///
/// The payload is real user info
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub payload: Payload,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        // ? I don't know what the code says here
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ApiError::Authenticate(AuthenticateError::MissingToken))?;
        // Decode the user data
        let token_data =
            jsonwebtoken::decode::<Self>(bearer.token(), &KEYS.decoding, &Validation::default())
                .map_err(|_| ApiError::Authenticate(AuthenticateError::InvalidToken))?;

        Ok(token_data.claims)
    }
}

impl From<Payload> for Claims {
    fn from(value: Payload) -> Self {
        Self::new(value.id, value.username)
    }
}

impl Claims {
    fn new(id: u64, username: String) -> Self {
        Self {
            exp: (chrono::Local::now() + chrono::Duration::seconds(*EXPIRE)).timestamp() as usize,
            iat: chrono::Local::now().timestamp() as usize,
            payload: Payload { id, username },
        }
    }
}

pub fn generate(payload: Payload) -> Result<String, Error> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &(Claims::from(payload)),
        &KEYS.encoding,
    )?)
}

pub fn decode(token: &str, secret: &str) -> Result<TokenData<Claims>, Error> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
}
