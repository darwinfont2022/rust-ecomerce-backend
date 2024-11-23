use std::collections::HashMap;
use std::fmt::{Display};
use actix_web::{error, HttpResponse, http::{
    header::ContentType,
    StatusCode
}, ResponseError};
use derive_more::{Display, Error};
use serde_json::json;

#[derive(Debug, Display, Error)]
pub enum  ApiException {
    #[display(fmt = "ApiException {}: {}", status, message)]
    Error {
        status: StatusCode,
        message: String,
        details: Option<HashMap<String, String>>
    },
}

impl ApiException {
    pub fn not_found(message: &str, details: Option<HashMap<String, String>>) -> HttpResponse {
        ApiException::Error {
            status: StatusCode::NOT_FOUND,
            message: format!("NOT FOUND {}", message),
            details,
        }.error_response()
    }

    pub fn internal_server_error(message: &String, details: Option<HashMap<String, String>>) -> HttpResponse {
        ApiException::Error {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("INTERNAL_SERVER_ERROR {}", message),
            details,
        }.error_response()
    }

    pub fn bad_request(message: &str, details: Option<HashMap<String, String>>) -> HttpResponse {
        ApiException::Error {
            status: StatusCode::BAD_REQUEST,
            message: String::from(message),
            details,
        }.error_response()
    }
}

impl error::ResponseError for ApiException {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiException::Error{status,..} => status,
        }
    }

    fn error_response(&self) -> HttpResponse {

        let payload = match self {
            ApiException::Error { status, message, details } => {
                let mut json_payload = json!({
                    "error": status.as_u16(),
                    "message": message,
                });

                if let Some(details) = details {
                    json_payload["details"] = json!(details);
                }

                json_payload
            }
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(payload.to_string())
    }
}