use axum::Json;
use jsonwebtoken::{encode, Header};
use model::common::jwt::{AuthBody, AuthError, AuthPayload, Claims, KEYS};
use model::response::Res;

pub async fn protected(claims: Claims) -> Res<Claims> {
    Res::ok_with_data(claims)
}

pub async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.id.is_empty() || payload.name.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    // if payload.id != "foo" || payload.name != "bar" {
    //     return Err(AuthError::WrongCredentials);
    // }
    let claims = Claims::new("sangshengjie".to_string());
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}
