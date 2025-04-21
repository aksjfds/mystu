use std::sync::LazyLock;
use std::time::Duration;

use crate::prelude::*;
use glacier::prelude::HeaderValue;
use serde::{Deserialize, Serialize};

///
///
///
///
///
///

pub const REFRESH_DURATION: Duration = Duration::from_secs(7 * 24 * 60 * 60);
pub const ACCESS_DURATION: Duration = Duration::from_secs(60 * 60);
pub const SIGN_UP_DURATION: Duration = Duration::from_secs(60 * 5);

pub static mut REFRESH_KEY: LazyLock<String> = LazyLock::new(|| {
    dotenv::dotenv().ok();

    let refresh_key = std::env::var("REFRESH_KEY").expect("REFRESH_KEY is not Provided");
    refresh_key
});

pub static mut ACCESS_KEY: LazyLock<String> = LazyLock::new(|| {
    dotenv::dotenv().ok();

    let access_key = std::env::var("ACCESS_KEY").expect("ACCESS_KEY is not Provided");
    access_key
});

pub static mut SIGN_UP_KEY: LazyLock<String> = LazyLock::new(|| {
    dotenv::dotenv().ok();

    let sign_up_key = std::env::var("SIGN_UP_KEY").expect("SIGN_UP_KEY is not Provided");
    sign_up_key
});

pub struct Key;
impl Key {
    pub fn access_key() -> &'static [u8] {
        unsafe { (&*ACCESS_KEY).as_bytes() }
    }

    pub fn refresh_key() -> &'static [u8] {
        unsafe { (&*REFRESH_KEY).as_bytes() }
    }

    pub fn sign_up_key() -> &'static [u8] {
        unsafe { (&*SIGN_UP_KEY).as_bytes() }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims<T> {
    pub payload: T,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct LoginPayload {
    pub email: String,
    pub username: String,
    pub role: i16,
}

pub fn decode<T>(token: &HeaderValue, key: &[u8]) -> Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    use jsonwebtoken::{DecodingKey, Validation};

    let token = token.to_str().map_err(|_e| Error::Status(401))?;

    let mut validation = Validation::default();
    validation.validate_exp = true;

    let payload =
        jsonwebtoken::decode::<Claims<T>>(token, &DecodingKey::from_secret(key), &validation)
            .map(|token| token.claims.payload)
            .map_err(|_e| Error::Status(401));

    payload
}

pub fn encode<T: Serialize>(
    payload: &T,
    key: &[u8],
    duration: std::time::Duration,
) -> Result<String> {
    use jsonwebtoken::{self, EncodingKey, Header};
    use std::time::{SystemTime, UNIX_EPOCH};

    let exp = SystemTime::now()
        .checked_add(duration)
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    jsonwebtoken::encode(
        &Header::default(),
        &Claims {
            payload: payload,
            exp: exp as usize,
        },
        &EncodingKey::from_secret(key),
    )
    .map_err(|e| {
        tracing::debug!("Error when encode JWT: {:#?}", e);
        Error::Status(533)
    })
}
