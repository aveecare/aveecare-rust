//! Cursor pagination.

use std::collections::VecDeque;
use std::future::Future;

use futures_util::stream::{self, Stream};
use serde::Deserialize;

use crate::error::Result;

/// One page of a list response.
///
/// To fetch the following page, pass [`Page::next_cursor`] as the `cursor`
/// list parameter, or use the resource's `list_auto_paging` stream.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct Page<T> {
    /// Always `list`.
    #[serde(default)]
    pub object: String,
    /// The objects on this page.
    #[serde(default = "Vec::new")]
    pub data: Vec<T>,
    /// Cursor for the next page; `None` on the last page.
    #[serde(default)]
    pub next_cursor: Option<String>,
    /// Whether another page exists.
    #[serde(default)]
    pub has_more: bool,
}

impl<T> Page<T> {
    /// Whether another page can be fetched.
    pub fn has_next_page(&self) -> bool {
        self.has_more && self.next_cursor.is_some()
    }
}

struct State<F> {
    buffer: VecDeque<F>,
    cursor: Option<String>,
    done: bool,
}

/// Turns a page fetcher into a stream of every item on every page.
pub(crate) fn auto_page<T, F, Fut>(fetch: F) -> impl Stream<Item = Result<T>>
where
    F: FnMut(Option<String>) -> Fut,
    Fut: Future<Output = Result<Page<T>>>,
{
    let state = State {
        buffer: VecDeque::new(),
        cursor: None,
        done: false,
    };
    stream::try_unfold((state, fetch), |(mut state, mut fetch)| async move {
        loop {
            if let Some(item) = state.buffer.pop_front() {
                return Ok(Some((item, (state, fetch))));
            }
            if state.done {
                return Ok(None);
            }
            let page = fetch(state.cursor.take()).await?;
            state.done = !page.has_next_page();
            state.cursor = page.next_cursor;
            state.buffer.extend(page.data);
        }
    })
}
