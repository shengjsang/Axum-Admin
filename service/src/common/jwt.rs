use jsonwebtoken::{encode, Header};
use model::common::jwt::{AuthBody, AuthError, AuthPayload, Claims, KEYS};

pub async fn authorize(payload: AuthPayload) -> Result<AuthBody, AuthError> {
    if payload.account.is_empty() {
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
    Ok(AuthBody::new(token))
}
