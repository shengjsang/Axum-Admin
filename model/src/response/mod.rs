use std::fmt::Debug;

use axum::body;
use axum::body::Full;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Res<T> {
    pub code: Option<u32>,
    pub data: Option<T>,
    pub msg: Option<String>,
}

impl<T> IntoResponse for Res<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn into_response(self) -> Response {
        let res = Self {
            code: self.code,
            data: self.data,
            msg: self.msg,
        };

        let res = match serde_json::to_string(&res) {
            Ok(res) => res,
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(body::boxed(Full::from(e.to_string())))
                    .unwrap()
            }
        };
        res.into_response()
    }
}

impl<T: Serialize> Res<T> {
    pub fn new(code: u32, data: T, msg: String) -> Self {
        Self {
            code: Some(code),
            data: Some(data),
            msg: Some(msg),
        }
    }

    pub fn ok() -> Self {
        Self {
            code: Some(200),
            data: None,
            msg: None,
        }
    }

    pub fn ok_with_data(data: T) -> Self {
        Self {
            code: Some(200),
            data: Some(data),
            msg: Some("Success".to_string()),
        }
    }

    pub fn ok_with_msg(msg: String) -> Self {
        Self {
            code: Some(200),
            data: None,
            msg: Some(msg),
        }
    }

    pub fn error(code: u32) -> Self {
        Self {
            code: Some(code),
            data: None,
            msg: None,
        }
    }

    pub fn error_with_data(code: u32, data: T) -> Self {
        Self {
            code: Some(code),
            data: Some(data),
            msg: Some("Error".to_string()),
        }
    }

    pub fn error_with_msg(code: u32, msg: String) -> Self {
        Self {
            code: Some(code),
            data: None,
            msg: Some(msg),
        }
    }
}
