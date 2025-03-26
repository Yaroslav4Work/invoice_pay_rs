use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Error {
    ValidationOneOfTheFieldsError {
        route: String,
        fields: Vec<String>,
    },
    ConversationError {
        route: String,
        from: String,
        to: String,
        msg: String,
    },
    RequestSendingError {
        route: String,
        msg: String,
    },
    RequestJsonSerializationError {
        route: String,
        from: String,
        msg: String,
    },
    ResponseJsonDeserializationError {
        route: String,
        to: String,
        msg: String,
    },
    ApiError {
        route: String,
        code: u16,
        msg: String,
        additions: Option<HashMap<String, Vec<String>>>,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ValidationOneOfTheFieldsError { route, fields } => {
                write!(
                    f,
                    "One of the fields {} must be at {}",
                    fields.join(", "),
                    route
                )
            }
            Error::ConversationError {
                route,
                from,
                to,
                msg,
            } => {
                write!(
                    f,
                    "Failed to convert {} into {} from request {}: {}",
                    from, to, route, msg
                )
            }
            Error::RequestSendingError { route, msg } => {
                write!(f, "Failed to send request to {}: {}", route, msg)
            }
            Error::RequestJsonSerializationError { route, from, msg } => {
                write!(
                    f,
                    "Failed to serialize to {} from request {}: {}",
                    route, from, msg
                )
            }
            Error::ResponseJsonDeserializationError { route, to, msg } => {
                write!(
                    f,
                    "Failed to deserialize from {} from request {}: {}",
                    to, route, msg
                )
            }
            Error::ApiError {
                route,
                code,
                msg,
                additions,
            } => {
                if let Some(additions) = additions {
                    write!(
                        f,
                        "Request to API {} was failed with code: {} - {}. Additions: {:?}",
                        route, code, msg, additions
                    )
                } else {
                    write!(
                        f,
                        "Request to API {} was failed with code: {} - {}",
                        route, code, msg
                    )
                }
            }
        }
    }
}

impl std::error::Error for Error {}
