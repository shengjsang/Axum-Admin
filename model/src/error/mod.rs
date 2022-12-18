use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error("a database  error occurred")]
    DbErr(#[from] DbErr),
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Anyhow(_) | Error::DbErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.status_code(), self.to_string()).into_response()
    }
}
