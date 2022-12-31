use anyhow::anyhow;
use jsonwebtoken::{encode, Header};
use model::common::jwt::{AuthBody, AuthError, AuthPayload, Claims, KEYS};

pub async fn authorize(payload: AuthPayload) -> anyhow::Result<AuthBody> {
    if payload.account.is_empty() {
        return Err(anyhow!(AuthError::MissingCredentials));
    }

    let claims = Claims::new(payload.id, payload.account);
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(AuthBody::new(
        token,
        claims.token_id,
        claims.iat,
        claims.exp,
    ))
}
