use dotenv::dotenv;
use std::{ops::Deref, sync::Arc};

#[derive(Debug, Clone)]
pub struct Config {
    pub inner: Arc<ConfigInner>,
}

impl Deref for Config {
    type Target = ConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            inner: Arc::new(ConfigInner {
                pwd_key: std::env::var("PWD_KEY").expect("PWD_KEY must be set"),
                db_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
                token_key: std::env::var("TOKEN_KEY").expect("TOKEN_KEY must be set"),
                token_duration_sec: std::env::var("TOKEN_DURATION_SEC")
                    .expect("TOKEN_DURATION_SEC must be set")
                    .parse()
                    .expect("TOKEN_DURATION_SEC must be a number"),
            }),
        }
    }
}

#[derive(Debug)]
pub struct ConfigInner {
    pub pwd_key: String,
    pub db_url: String,
    pub token_key: String,
    pub token_duration_sec: u64,
}
