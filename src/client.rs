//! The asynchronous client.

use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT,
};
use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{ApiErrorDetails, Error, Result};
use crate::pagination::Page;

/// Default API origin.
pub const DEFAULT_BASE_URL: &str = "https://api.aveecare.com";
/// Default per-attempt timeout.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);
/// Default number of retries for 429, 5xx and network errors.
pub const DEFAULT_MAX_RETRIES: u32 = 2;
const MAX_RETRY_DELAY: Duration = Duration::from_secs(60);
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Per-request overrides.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct RequestOptions {
    /// Idempotency key for a POST. One is generated when omitted.
    pub idempotency_key: Option<String>,
    /// Per-attempt timeout for this request.
    pub timeout: Option<Duration>,
    /// Retries for this request.
    pub max_retries: Option<u32>,
}

impl RequestOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn idempotency_key(mut self, key: impl Into<String>) -> Self {
        self.idempotency_key = Some(key.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }
}

struct Inner {
    http: reqwest::Client,
    api_key: String,
    base_url: String,
    timeout: Duration,
    max_retries: u32,
    default_headers: HeaderMap,
}

/// The AveeCare API client. Cheap to clone; clones share one connection pool.
///
/// ```no_run
/// # async fn run() -> Result<(), aveecare::Error> {
/// let avee = aveecare::AveeCare::new(std::env::var("AVEECARE_API_KEY").unwrap());
/// let me = avee.me().retrieve().await?;
/// println!("{}", me.name.unwrap_or_default());
/// # Ok(()) }
/// ```
#[derive(Clone)]
pub struct AveeCare {
    inner: Arc<Inner>,
}

impl std::fmt::Debug for AveeCare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AveeCare")
            .field("base_url", &self.inner.base_url)
            .field("timeout", &self.inner.timeout)
            .field("max_retries", &self.inner.max_retries)
            .finish_non_exhaustive()
    }
}

impl AveeCare {
    /// A client with default settings. The base URL comes from
    /// `AVEECARE_BASE_URL` when set, otherwise `https://api.aveecare.com`.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::builder()
            .api_key(api_key)
            .build()
            .expect("a client with an API key always builds")
    }

    /// A client configured from `AVEECARE_API_KEY` and `AVEECARE_BASE_URL`.
    pub fn from_env() -> Result<Self> {
        Self::builder().build()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// The API origin requests go to.
    pub fn base_url(&self) -> &str {
        &self.inner.base_url
    }

    pub(crate) async fn request<T, B>(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, String)],
        body: Option<&B>,
        idempotent: bool,
        options: &RequestOptions,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let inner = &self.inner;
        let url = format!("{}{}", inner.base_url, path);
        let mut headers = inner.default_headers.clone();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", inner.api_key)).map_err(|_| {
                Error::Config("the API key contains characters not allowed in a header".into())
            })?,
        );
        headers.insert(
            USER_AGENT,
            header_value(&format!("aveecare-rust/{VERSION}")),
        );
        let payload = match body {
            Some(b) => {
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                Some(serde_json::to_vec(b).map_err(|e| {
                    Error::Config(format!("could not encode the request body: {e}"))
                })?)
            }
            None => None,
        };
        if method == Method::POST || idempotent {
            let key = options
                .idempotency_key
                .clone()
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
            headers.insert(
                HeaderName::from_static("idempotency-key"),
                header_value(&key),
            );
        }
        let timeout = options.timeout.unwrap_or(inner.timeout);
        let max_retries = options.max_retries.unwrap_or(inner.max_retries);

        let mut attempt: u32 = 0;
        loop {
            let mut req = inner
                .http
                .request(method.clone(), &url)
                .headers(headers.clone())
                .timeout(timeout);
            if !query.is_empty() {
                req = req.query(query);
            }
            if let Some(p) = &payload {
                req = req.body(p.clone());
            }
            let response = match req.send().await {
                Ok(r) => r,
                Err(e) => {
                    if attempt < max_retries {
                        tokio::time::sleep(backoff_delay(attempt)).await;
                        attempt += 1;
                        continue;
                    }
                    return Err(if e.is_timeout() {
                        Error::Timeout
                    } else {
                        Error::Connection(e)
                    });
                }
            };
            let status = response.status();
            if should_retry(status) && attempt < max_retries {
                let hinted = retry_after(response.headers());
                tokio::time::sleep(
                    hinted
                        .unwrap_or_else(|| backoff_delay(attempt))
                        .min(MAX_RETRY_DELAY),
                )
                .await;
                attempt += 1;
                continue;
            }
            let request_id = response
                .headers()
                .get("request-id")
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned);
            let retry_after_hint = retry_after(response.headers());
            let bytes = match response.bytes().await {
                Ok(b) => b,
                Err(e) => {
                    if attempt < max_retries {
                        tokio::time::sleep(backoff_delay(attempt)).await;
                        attempt += 1;
                        continue;
                    }
                    return Err(if e.is_timeout() {
                        Error::Timeout
                    } else {
                        Error::Connection(e)
                    });
                }
            };
            if status.is_success() {
                let raw: &[u8] = if bytes.is_empty() { b"null" } else { &bytes };
                return serde_json::from_slice(raw).map_err(|e| Error::Decode {
                    message: e.to_string(),
                    request_id,
                });
            }
            return Err(Error::from_response(parse_error(
                status,
                &bytes,
                request_id,
                retry_after_hint,
            )));
        }
    }

    pub(crate) async fn get_page<T: DeserializeOwned>(
        &self,
        path: &str,
        query: Vec<(&'static str, String)>,
        options: &RequestOptions,
    ) -> Result<Page<T>> {
        self.request::<Page<T>, ()>(Method::GET, path, &query, None, false, options)
            .await
    }
}

fn header_value(s: &str) -> HeaderValue {
    HeaderValue::from_str(s).unwrap_or_else(|_| HeaderValue::from_static("invalid"))
}

fn should_retry(status: StatusCode) -> bool {
    status == StatusCode::REQUEST_TIMEOUT
        || status == StatusCode::TOO_MANY_REQUESTS
        || status.is_server_error()
}

/// Jittered exponential backoff: 0.5s, 1s, 2s, ... capped at 8s, minus up to 25%.
pub(crate) fn backoff_delay(attempt: u32) -> Duration {
    let base = (0.5_f64 * 2_f64.powi(attempt.min(10) as i32)).min(8.0);
    Duration::from_secs_f64(base * (1.0 - fastrand::f64() * 0.25))
}

/// `Retry-After` in seconds. (HTTP-date values fall back to backoff.)
fn retry_after(headers: &HeaderMap) -> Option<Duration> {
    let raw = headers.get("retry-after")?.to_str().ok()?.trim();
    let secs: f64 = raw.parse().ok()?;
    if secs.is_finite() && secs >= 0.0 {
        Some(Duration::from_secs_f64(secs))
    } else {
        None
    }
}

#[derive(serde::Deserialize)]
struct ErrorEnvelope {
    error: Option<ErrorBody>,
}

#[derive(serde::Deserialize)]
struct ErrorBody {
    #[serde(rename = "type")]
    error_type: Option<String>,
    code: Option<String>,
    message: Option<String>,
    request_id: Option<String>,
}

fn parse_error(
    status: StatusCode,
    bytes: &[u8],
    request_id: Option<String>,
    retry_after: Option<Duration>,
) -> ApiErrorDetails {
    let body = serde_json::from_slice::<ErrorEnvelope>(bytes)
        .ok()
        .and_then(|e| e.error);
    let (error_type, code, message, body_request_id) = match body {
        Some(b) => (b.error_type, b.code, b.message, b.request_id),
        None => (None, None, None, None),
    };
    ApiErrorDetails {
        status: status.as_u16(),
        code,
        error_type,
        message: message
            .unwrap_or_else(|| format!("Request failed with status {}", status.as_u16())),
        request_id: body_request_id.or(request_id),
        retry_after,
    }
}

/// Builds an [`AveeCare`] client.
#[derive(Debug, Default)]
pub struct ClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
    http: Option<reqwest::Client>,
    default_headers: HeaderMap,
}

impl ClientBuilder {
    /// Your secret key, `ak_<keyId>_<secret>`. Defaults to `AVEECARE_API_KEY`.
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Defaults to `AVEECARE_BASE_URL`, then `https://api.aveecare.com`.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Per-attempt timeout. Default 60 seconds.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Retries for 429, 5xx and network errors. Default 2.
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }

    /// Use your own `reqwest::Client` (proxies, custom TLS, ...).
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http = Some(client);
        self
    }

    /// A header sent with every request.
    pub fn default_header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.default_headers.insert(name, value);
        self
    }

    pub fn build(self) -> Result<AveeCare> {
        let env = |name: &str| {
            std::env::var(name)
                .ok()
                .map(|v| v.trim().to_owned())
                .filter(|v| !v.is_empty())
        };
        let api_key = self
            .api_key
            .or_else(|| env("AVEECARE_API_KEY"))
            .ok_or_else(|| {
                Error::Config(
                    "no API key provided: call .api_key() or set AVEECARE_API_KEY \
                 (see https://www.aveecare.com/docs/api/authentication)"
                        .into(),
                )
            })?;
        let base_url = self
            .base_url
            .or_else(|| env("AVEECARE_BASE_URL"))
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_owned())
            .trim_end_matches('/')
            .to_owned();
        Ok(AveeCare {
            inner: Arc::new(Inner {
                http: self.http.unwrap_or_default(),
                api_key,
                base_url,
                timeout: self.timeout.unwrap_or(DEFAULT_TIMEOUT),
                max_retries: self.max_retries.unwrap_or(DEFAULT_MAX_RETRIES),
                default_headers: self.default_headers,
            }),
        })
    }
}

/// Percent-encodes one path segment.
pub(crate) fn encode_path(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len());
    for b in segment.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
