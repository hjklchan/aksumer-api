use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

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

pub fn generate(secret: &str, payload: Payload) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &(Claims::from(payload)),
        &encoding_key,
    )?)
}

pub fn decode(token: &str, secret: &str) -> Result<TokenData<Claims>, Error> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
}
