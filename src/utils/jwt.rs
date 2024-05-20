use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::error::api_error::{ApiError, AuthenticateError};

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("AUTH_SECRET").expect("AUTH_SECRET must be set");
    Keys::new(secret.as_bytes())
});

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub id: u64,
    pub username: String,
}

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
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ApiError::Authenticate(AuthenticateError::MissingToken))?;
        // Decode the user data
        let token_data =
            jsonwebtoken::decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
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
            exp: (chrono::Local::now() + chrono::Duration::days(30)).timestamp() as usize,
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
