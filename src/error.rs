//! Errors returned by the client.

use std::fmt;
use std::time::Duration;

/// Details of an error response from the API.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct ApiErrorDetails {
    /// HTTP status code.
    pub status: u16,
    /// Machine-readable code such as `not_found` or `insufficient_scope`.
    pub code: Option<String>,
    /// Broad category such as `invalid_request_error`.
    pub error_type: Option<String>,
    /// Human-readable explanation.
    pub message: String,
    /// The `Request-Id` of the failed request. Quote it when contacting support.
    pub request_id: Option<String>,
    /// How long the API asked us to wait (from `Retry-After`), if given.
    pub retry_after: Option<Duration>,
}

/// Every error this library returns.
///
/// HTTP errors are chosen by status: 400/422 [`Error::InvalidRequest`],
/// 401 [`Error::Authentication`], 403 [`Error::Permission`], 404
/// [`Error::NotFound`], 409 [`Error::Conflict`], 429 [`Error::RateLimit`],
/// and anything else (including 5xx) [`Error::Api`]. Their details are boxed
/// to keep `Result<T, Error>` small.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    InvalidRequest(Box<ApiErrorDetails>),
    Authentication(Box<ApiErrorDetails>),
    Permission(Box<ApiErrorDetails>),
    NotFound(Box<ApiErrorDetails>),
    Conflict(Box<ApiErrorDetails>),
    RateLimit(Box<ApiErrorDetails>),
    Api(Box<ApiErrorDetails>),
    /// The request never got a response (DNS, TLS, connection reset, ...).
    Connection(reqwest::Error),
    /// The request exceeded its timeout.
    Timeout,
    /// The response body was not the JSON we expected.
    Decode {
        message: String,
        request_id: Option<String>,
    },
    /// The client was misconfigured (for example, no API key).
    Config(String),
}

impl Error {
    /// The API error details, when the error came from a response.
    pub fn details(&self) -> Option<&ApiErrorDetails> {
        match self {
            Error::InvalidRequest(d)
            | Error::Authentication(d)
            | Error::Permission(d)
            | Error::NotFound(d)
            | Error::Conflict(d)
            | Error::RateLimit(d)
            | Error::Api(d) => Some(d),
            _ => None,
        }
    }

    /// HTTP status, when the error came from a response.
    pub fn status(&self) -> Option<u16> {
        self.details().map(|d| d.status)
    }

    /// Machine-readable error code, such as `not_found`.
    pub fn code(&self) -> Option<&str> {
        self.details().and_then(|d| d.code.as_deref())
    }

    /// The `Request-Id` of the failed request.
    pub fn request_id(&self) -> Option<&str> {
        match self {
            Error::Decode { request_id, .. } => request_id.as_deref(),
            _ => self.details().and_then(|d| d.request_id.as_deref()),
        }
    }

    pub(crate) fn from_response(details: ApiErrorDetails) -> Self {
        let details = Box::new(details);
        match details.status {
            400 | 422 => Error::InvalidRequest(details),
            401 => Error::Authentication(details),
            403 => Error::Permission(details),
            404 => Error::NotFound(details),
            409 => Error::Conflict(details),
            429 => Error::RateLimit(details),
            _ => Error::Api(details),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Connection(e) => write!(f, "could not reach the AveeCare API: {e}"),
            Error::Timeout => write!(f, "request timed out"),
            Error::Decode { message, .. } => write!(f, "unexpected response: {message}"),
            Error::Config(message) => write!(f, "{message}"),
            other => {
                let d = other.details().expect("API error variants carry details");
                write!(f, "{} (status {}", d.message, d.status)?;
                if let Some(code) = &d.code {
                    write!(f, ", code {code}")?;
                }
                if let Some(id) = &d.request_id {
                    write!(f, ", request {id}")?;
                }
                write!(f, ")")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Connection(e) => Some(e),
            _ => None,
        }
    }
}

/// Shorthand for `std::result::Result<T, aveecare::Error>`.
pub type Result<T> = std::result::Result<T, Error>;
