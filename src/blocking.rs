//! A synchronous client, enabled with the `blocking` feature.
//!
//! It drives the async client on its own single-threaded Tokio runtime, so do
//! not call it from inside an async runtime (Tokio panics if you do); use
//! [`crate::AveeCare`] there instead.
//!
//! ```no_run
//! use aveecare::blocking::AveeCare;
//! use aveecare::PatientListParams;
//!
//! let avee = AveeCare::new(std::env::var("AVEECARE_API_KEY").unwrap());
//! for patient in avee.patients().list_auto_paging(PatientListParams::new()) {
//!     println!("{}", patient?.id);
//! }
//! # Ok::<(), aveecare::Error>(())
//! ```

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use futures_util::{Stream, StreamExt};

use crate::error::Result;

pub use crate::generated::blocking::*;

/// The synchronous AveeCare API client. Cheap to clone.
#[derive(Clone, Debug)]
pub struct AveeCare {
    pub(crate) inner: crate::AveeCare,
    rt: Arc<tokio::runtime::Runtime>,
}

impl AveeCare {
    /// A client with default settings; see [`crate::AveeCare::new`].
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::from_async(crate::AveeCare::new(api_key))
    }

    /// A client configured from `AVEECARE_API_KEY` and `AVEECARE_BASE_URL`.
    pub fn from_env() -> Result<Self> {
        Ok(Self::from_async(crate::AveeCare::from_env()?))
    }

    /// Wraps an async client built with [`crate::AveeCare::builder`].
    pub fn from_async(client: crate::AveeCare) -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to start the Tokio runtime for the blocking client");
        Self {
            inner: client,
            rt: Arc::new(rt),
        }
    }

    pub(crate) fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.rt.block_on(future)
    }

    pub(crate) fn iter<T, S>(&self, stream: S) -> BlockingIter<T>
    where
        S: Stream<Item = Result<T>> + Send + 'static,
    {
        BlockingIter {
            rt: self.rt.clone(),
            stream: Box::pin(stream),
        }
    }
}

/// Iterator over every item of every page, returned by `list_auto_paging`.
pub struct BlockingIter<T> {
    rt: Arc<tokio::runtime::Runtime>,
    stream: Pin<Box<dyn Stream<Item = Result<T>> + Send>>,
}

impl<T> Iterator for BlockingIter<T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let stream = &mut self.stream;
        self.rt.block_on(stream.next())
    }
}

impl<T> std::fmt::Debug for BlockingIter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockingIter").finish_non_exhaustive()
    }
}
