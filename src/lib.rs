//! Official Rust library for the [AveeCare API](https://www.aveecare.com/docs/api).
//!
//! ```no_run
//! use aveecare::{AveeCare, PatientLifecycleStage, PatientListParams};
//!
//! # async fn run() -> Result<(), aveecare::Error> {
//! let avee = AveeCare::new(std::env::var("AVEECARE_API_KEY").unwrap());
//! let page = avee
//!     .patients()
//!     .list(PatientListParams::new().lifecycle_stage(PatientLifecycleStage::Client).limit(25))
//!     .await?;
//! for patient in page.data {
//!     println!("{} {}", patient.first_name.unwrap_or_default(), patient.last_name.unwrap_or_default());
//! }
//! # Ok(()) }
//! ```
//!
//! Every list call has a `list_auto_paging` twin that returns a
//! [`futures_util::Stream`] of every item across pages. Requests retry 429,
//! 5xx and network errors with jittered exponential backoff (honoring
//! `Retry-After`), and every POST carries an `Idempotency-Key`.
//!
//! Enable the `blocking` feature for a synchronous client in [`blocking`].

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

mod client;
mod error;
mod generated;
mod pagination;

#[cfg(feature = "blocking")]
pub mod blocking;

pub use client::{
    AveeCare, ClientBuilder, RequestOptions, DEFAULT_BASE_URL, DEFAULT_MAX_RETRIES, DEFAULT_TIMEOUT,
};
pub use error::{ApiErrorDetails, Error, Result};
pub use generated::models::*;
pub use generated::resources::*;
pub use pagination::Page;

/// Re-exported so callers can use `StreamExt` without adding the crate.
pub use futures_util;

/// Compiles every Rust example in README.md as a doctest.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
pub struct ReadmeDoctests;
