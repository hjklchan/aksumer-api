use std::env;

use dotenvy::dotenv;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ENV: Env = Env::init();
}

#[derive(Debug, Clone)]
pub struct Env {
    pub server: Server,
    pub database: Database,
    pub auth: Auth,
}

impl Env {
    fn init() -> Env {
        dotenv().ok();

        Env {
            server: Server(env::var("SERVER_ADDR").unwrap_or("default".into())),
            // DATABASE_URL default empty
            database: Database(env::var("DATABASE_URL").unwrap_or("".into())),
            auth: Auth {
                secret: env::var("AUTH_SECRET").unwrap_or("aksumer-api".into()),
                expire: env::var("AUTH_EXPIRE")
                    .unwrap_or("36000".into())
                    .parse()
                    .unwrap(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Server(pub String);

#[derive(Debug, Clone)]
pub struct Database(pub String);

#[derive(Debug, Clone)]
pub struct Auth {
    pub secret: String,
    pub expire: u64,
}
