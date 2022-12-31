use axum::extract::{FromRequestParts, TypedHeader};
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::{async_trait, RequestPartsExt};
use chrono::{Duration, Local, TimeZone};
use configs::CFG;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Debug, Display};
use thiserror::Error;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = &CFG.jwt.secret;
    Keys::new(secret.as_bytes())
});

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

impl AuthBody {
    pub fn new(access_token: String, token_id: String, iat: i64, exp: i64) -> Self {
        let fmt = "%Y-%m-%d %H:%M:%S";
        Self {
            token_id,
            token: access_token,
            token_type: "Bearer".to_string(),
            iat: format!("{}", Local.timestamp_opt(iat, 0).unwrap().format(fmt)),
            exp: format!("{}", Local.timestamp_opt(exp, 0).unwrap().format(fmt)),
        }
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {}\nAccount: {}\nExp: {}",
            self.id, self.account, self.exp
        )
    }
}

impl Claims {
    pub fn new(id: String, account: String) -> Self {
        let token_id = scru128::new_string();
        let iat = Local::now();
        let exp = iat + Duration::hours(24 * &CFG.jwt.exp);

        Self {
            id,
            token_id,
            account,
            exp: exp.timestamp(),
            iat: iat.timestamp(),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::Unknown => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub id: String,
    pub token_id: String,
    pub account: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthPayload {
    pub id: String,
    pub account: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthBody {
    pub token_id: String,
    pub token_type: String,
    pub iat: String,
    pub exp: String,
    pub token: String,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Token creation error")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Unknown")]
    Unknown,
}
