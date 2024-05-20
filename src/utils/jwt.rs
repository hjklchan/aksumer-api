use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    id: u64,
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims<T> {
    pub exp: usize,
    pub iat: usize,
    pub payload: Payload,
    _unused: std::marker::PhantomData<T>
}

impl<T> Claims<T> {
    // TODO
}
