use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, strum_macros::AsRefStr, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail,

    // Model errors
    TicketDeleteFailIdNotFound { id: u64 },

    // Auth errors
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "->> Error: {:?}", self)
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:12} - {self:?}", "ERROR");

        // Create a placeholder response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response
        response.extensions_mut().insert(self);

        response
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVAID_PARAMS,
    SERVER_ERROR,
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            // Login
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

            // Auth
            Self::AuthFailCtxNotInRequestExt
            | Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

            // Model
            Self::TicketDeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVAID_PARAMS)
            }

            // Fallback
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVER_ERROR),
        }
    }
}
