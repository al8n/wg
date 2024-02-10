/*
 * Copyright 2021 Al Liu (https://github.com/al8n). Licensed under MIT OR Apache-2.0.
 *
 *
 *
 * Copyright 2021 AwaitGroup authors (https://github.com/ibraheemdev/awaitgroup). Licensed under MIT.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
//! <div align="center">
//! <h1>wg</h1>
//! </div>
//! <div align="center">
//!
//! Golang like WaitGroup implementation for sync/async Rust.
//!
//! [<img alt="github" src="https://img.shields.io/badge/GITHUB-wg-8da0cb?style=for-the-badge&logo=Github" height="22">][Github-url]
//! [<img alt="Build" src="https://img.shields.io/github/workflow/status/al8n/wg/CI/main?logo=Github-Actions&style=for-the-badge" height="22">][CI-url]
//! [<img alt="codecov" src="https://img.shields.io/codecov/c/gh/al8n/wg?style=for-the-badge&token=0WQ0RUeAz0&logo=codecov" height="22">][codecov-url]
//!
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-wg-66c2a5?style=for-the-badge&labelColor=555555&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">][doc-url]
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/wg?style=for-the-badge&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iaXNvLTg4NTktMSI/Pg0KPCEtLSBHZW5lcmF0b3I6IEFkb2JlIElsbHVzdHJhdG9yIDE5LjAuMCwgU1ZHIEV4cG9ydCBQbHVnLUluIC4gU1ZHIFZlcnNpb246IDYuMDAgQnVpbGQgMCkgIC0tPg0KPHN2ZyB2ZXJzaW9uPSIxLjEiIGlkPSJMYXllcl8xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIiB4PSIwcHgiIHk9IjBweCINCgkgdmlld0JveD0iMCAwIDUxMiA1MTIiIHhtbDpzcGFjZT0icHJlc2VydmUiPg0KPGc+DQoJPGc+DQoJCTxwYXRoIGQ9Ik0yNTYsMEwzMS41MjgsMTEyLjIzNnYyODcuNTI4TDI1Niw1MTJsMjI0LjQ3Mi0xMTIuMjM2VjExMi4yMzZMMjU2LDB6IE0yMzQuMjc3LDQ1Mi41NjRMNzQuOTc0LDM3Mi45MTNWMTYwLjgxDQoJCQlsMTU5LjMwMyw3OS42NTFWNDUyLjU2NHogTTEwMS44MjYsMTI1LjY2MkwyNTYsNDguNTc2bDE1NC4xNzQsNzcuMDg3TDI1NiwyMDIuNzQ5TDEwMS44MjYsMTI1LjY2MnogTTQzNy4wMjYsMzcyLjkxMw0KCQkJbC0xNTkuMzAzLDc5LjY1MVYyNDAuNDYxbDE1OS4zMDMtNzkuNjUxVjM3Mi45MTN6IiBmaWxsPSIjRkZGIi8+DQoJPC9nPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPC9zdmc+DQo=" height="22">][crates-url]
//! [<img alt="crates.io" src="https://img.shields.io/crates/d/wg?color=critical&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBzdGFuZGFsb25lPSJubyI/PjwhRE9DVFlQRSBzdmcgUFVCTElDICItLy9XM0MvL0RURCBTVkcgMS4xLy9FTiIgImh0dHA6Ly93d3cudzMub3JnL0dyYXBoaWNzL1NWRy8xLjEvRFREL3N2ZzExLmR0ZCI+PHN2ZyB0PSIxNjQ1MTE3MzMyOTU5IiBjbGFzcz0iaWNvbiIgdmlld0JveD0iMCAwIDEwMjQgMTAyNCIgdmVyc2lvbj0iMS4xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHAtaWQ9IjM0MjEiIGRhdGEtc3BtLWFuY2hvci1pZD0iYTMxM3guNzc4MTA2OS4wLmkzIiB3aWR0aD0iNDgiIGhlaWdodD0iNDgiIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIj48ZGVmcz48c3R5bGUgdHlwZT0idGV4dC9jc3MiPjwvc3R5bGU+PC9kZWZzPjxwYXRoIGQ9Ik00NjkuMzEyIDU3MC4yNHYtMjU2aDg1LjM3NnYyNTZoMTI4TDUxMiA3NTYuMjg4IDM0MS4zMTIgNTcwLjI0aDEyOHpNMTAyNCA2NDAuMTI4QzEwMjQgNzgyLjkxMiA5MTkuODcyIDg5NiA3ODcuNjQ4IDg5NmgtNTEyQzEyMy45MDQgODk2IDAgNzYxLjYgMCA1OTcuNTA0IDAgNDUxLjk2OCA5NC42NTYgMzMxLjUyIDIyNi40MzIgMzAyLjk3NiAyODQuMTYgMTk1LjQ1NiAzOTEuODA4IDEyOCA1MTIgMTI4YzE1Mi4zMiAwIDI4Mi4xMTIgMTA4LjQxNiAzMjMuMzkyIDI2MS4xMkM5NDEuODg4IDQxMy40NCAxMDI0IDUxOS4wNCAxMDI0IDY0MC4xOTJ6IG0tMjU5LjItMjA1LjMxMmMtMjQuNDQ4LTEyOS4wMjQtMTI4Ljg5Ni0yMjIuNzItMjUyLjgtMjIyLjcyLTk3LjI4IDAtMTgzLjA0IDU3LjM0NC0yMjQuNjQgMTQ3LjQ1NmwtOS4yOCAyMC4yMjQtMjAuOTI4IDIuOTQ0Yy0xMDMuMzYgMTQuNC0xNzguMzY4IDEwNC4zMi0xNzguMzY4IDIxNC43MiAwIDExNy45NTIgODguODMyIDIxNC40IDE5Ni45MjggMjE0LjRoNTEyYzg4LjMyIDAgMTU3LjUwNC03NS4xMzYgMTU3LjUwNC0xNzEuNzEyIDAtODguMDY0LTY1LjkyLTE2NC45MjgtMTQ0Ljk2LTE3MS43NzZsLTI5LjUwNC0yLjU2LTUuODg4LTMwLjk3NnoiIGZpbGw9IiNmZmZmZmYiIHAtaWQ9IjM0MjIiIGRhdGEtc3BtLWFuY2hvci1pZD0iYTMxM3guNzc4MTA2OS4wLmkwIiBjbGFzcz0iIj48L3BhdGg+PC9zdmc+&style=for-the-badge" height="22">][crates-url]
//!
//! <img alt="license" src="https://img.shields.io/badge/License-Apache%202.0/MIT-blue.svg?style=for-the-badge&fontColor=white&logoColor=f5c076&logo=data:image/svg+xml;base64,PCFET0NUWVBFIHN2ZyBQVUJMSUMgIi0vL1czQy8vRFREIFNWRyAxLjEvL0VOIiAiaHR0cDovL3d3dy53My5vcmcvR3JhcGhpY3MvU1ZHLzEuMS9EVEQvc3ZnMTEuZHRkIj4KDTwhLS0gVXBsb2FkZWQgdG86IFNWRyBSZXBvLCB3d3cuc3ZncmVwby5jb20sIFRyYW5zZm9ybWVkIGJ5OiBTVkcgUmVwbyBNaXhlciBUb29scyAtLT4KPHN2ZyBmaWxsPSIjZmZmZmZmIiBoZWlnaHQ9IjgwMHB4IiB3aWR0aD0iODAwcHgiIHZlcnNpb249IjEuMSIgaWQ9IkNhcGFfMSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayIgdmlld0JveD0iMCAwIDI3Ni43MTUgMjc2LjcxNSIgeG1sOnNwYWNlPSJwcmVzZXJ2ZSIgc3Ryb2tlPSIjZmZmZmZmIj4KDTxnIGlkPSJTVkdSZXBvX2JnQ2FycmllciIgc3Ryb2tlLXdpZHRoPSIwIi8+Cg08ZyBpZD0iU1ZHUmVwb190cmFjZXJDYXJyaWVyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiLz4KDTxnIGlkPSJTVkdSZXBvX2ljb25DYXJyaWVyIj4gPGc+IDxwYXRoIGQ9Ik0xMzguMzU3LDBDNjIuMDY2LDAsMCw2Mi4wNjYsMCwxMzguMzU3czYyLjA2NiwxMzguMzU3LDEzOC4zNTcsMTM4LjM1N3MxMzguMzU3LTYyLjA2NiwxMzguMzU3LTEzOC4zNTcgUzIxNC42NDgsMCwxMzguMzU3LDB6IE0xMzguMzU3LDI1OC43MTVDNzEuOTkyLDI1OC43MTUsMTgsMjA0LjcyMywxOCwxMzguMzU3UzcxLjk5MiwxOCwxMzguMzU3LDE4IHMxMjAuMzU3LDUzLjk5MiwxMjAuMzU3LDEyMC4zNTdTMjA0LjcyMywyNTguNzE1LDEzOC4zNTcsMjU4LjcxNXoiLz4gPHBhdGggZD0iTTE5NC43OTgsMTYwLjkwM2MtNC4xODgtMi42NzctOS43NTMtMS40NTQtMTIuNDMyLDIuNzMyYy04LjY5NCwxMy41OTMtMjMuNTAzLDIxLjcwOC0zOS42MTQsMjEuNzA4IGMtMjUuOTA4LDAtNDYuOTg1LTIxLjA3OC00Ni45ODUtNDYuOTg2czIxLjA3Ny00Ni45ODYsNDYuOTg1LTQ2Ljk4NmMxNS42MzMsMCwzMC4yLDcuNzQ3LDM4Ljk2OCwyMC43MjMgYzIuNzgyLDQuMTE3LDguMzc1LDUuMjAxLDEyLjQ5NiwyLjQxOGM0LjExOC0yLjc4Miw1LjIwMS04LjM3NywyLjQxOC0xMi40OTZjLTEyLjExOC0xNy45MzctMzIuMjYyLTI4LjY0NS01My44ODItMjguNjQ1IGMtMzUuODMzLDAtNjQuOTg1LDI5LjE1Mi02NC45ODUsNjQuOTg2czI5LjE1Miw2NC45ODYsNjQuOTg1LDY0Ljk4NmMyMi4yODEsMCw0Mi43NTktMTEuMjE4LDU0Ljc3OC0zMC4wMDkgQzIwMC4yMDgsMTY5LjE0NywxOTguOTg1LDE2My41ODIsMTk0Ljc5OCwxNjAuOTAzeiIvPiA8L2c+IDwvZz4KDTwvc3ZnPg==" height="22">
//!
//! </div>
//!
//! ## Installation
//! ```toml
//! [dependencies]
//! wg = "0.4"
//! ```
//!
//! ## Example
//!
//! ### Sync
//! ```rust,ignore
//! use wg::WaitGroup;
//! use std::sync::Arc;
//! use std::sync::atomic::{AtomicUsize, Ordering};
//! use std::time::Duration;
//! use std::thread::{spawn, sleep};
//!
//! let wg = WaitGroup::new();
//! let ctr = Arc::new(AtomicUsize::new(0));
//!
//! for _ in 0..5 {
//!   let ctrx = ctr.clone();
//!   let t_wg = wg.add(1);
//!   spawn(move || {
//!     // mock some time consuming task
//!     sleep(Duration::from_millis(50));
//!     ctrx.fetch_add(1, Ordering::Relaxed);
//!
//!     // mock task is finished
//!     t_wg.done();
//!   });
//! }
//!
//! wg.wait();
//! assert_eq!(ctr.load(Ordering::Relaxed), 5);
//!
//! ```
//!
//! ### Async
//! ```rust,ignore
//!
//! use wg::AsyncWaitGroup;
//! use std::sync::Arc;
//! use std::sync::atomic::{AtomicUsize, Ordering};
//! use tokio::{spawn, time::{sleep, Duration}};
//!
//! #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
//! async fn main() {
//!     let wg = AsyncWaitGroup::new();
//!     let ctr = Arc::new(AtomicUsize::new(0));
//!
//!     for _ in 0..5 {
//!         let ctrx = ctr.clone();
//!         let t_wg = wg.add(1);
//!         spawn(async move {
//!             // mock some time consuming task
//!             sleep(Duration::from_millis(50)).await;
//!             ctrx.fetch_add(1, Ordering::Relaxed);
//!
//!             // mock task is finished
//!             t_wg.done();
//!         });
//!     }
//!
//!     wg.wait().await;
//!     assert_eq!(ctr.load(Ordering::Relaxed), 5);
//! }
//! ```
//!
//! ## Acknowledgements
//! - Inspired by Golang sync.WaitGroup, [ibraheemdev's `AwaitGroup`] and [`crossbeam_utils::WaitGroup`].
//!
//!
//!
//! [ibraheemdev's `AwaitGroup`]: https://github.com/ibraheemdev/awaitgroup
//! [`crossbeam_utils::WaitGroup`]: https://docs.rs/crossbeam/0.8.1/crossbeam/sync/struct.WaitGroup.html
//! [Github-url]: https://github.com/al8n/wg/
//! [CI-url]: https://github.com/al8n/wg/actions/workflows/ci.yml
//! [doc-url]: https://docs.rs/wg
//! [crates-url]: https://crates.io/crates/wg
//! [codecov-url]: https://app.codecov.io/gh/al8n/wg/
//! [license-url]: https://opensource.org/licenses/Apache-2.0
//! [rustc-url]: https://github.com/rust-lang/rust/blob/master/RELEASES.md
//! [license-apache-url]: https://opensource.org/licenses/Apache-2.0
//! [license-mit-url]: https://opensource.org/licenses/MIT
//! [rustc-image]: https://img.shields.io/badge/rustc-1.56.0%2B-orange.svg?style=for-the-badge&logo=Rust
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

trait Mu {
    type Guard<'a>
    where
        Self: 'a;
    fn lock_me(&self) -> Self::Guard<'_>;
}

#[cfg(feature = "parking_lot")]
impl<T: ?Sized> Mu for parking_lot::Mutex<T> {
    type Guard<'a> = parking_lot::MutexGuard<'a, T> where Self: 'a;

    fn lock_me(&self) -> Self::Guard<'_> {
        self.lock()
    }
}

#[cfg(not(feature = "parking_lot"))]
impl<T: ?Sized> Mu for std::sync::Mutex<T> {
    type Guard<'a> = std::sync::MutexGuard<'a, T> where Self: 'a;

    fn lock_me(&self) -> Self::Guard<'_> {
        self.lock().unwrap()
    }
}

#[cfg(feature = "parking_lot")]
use parking_lot::{Condvar, Mutex};
#[cfg(not(feature = "triomphe"))]
use std::sync::Arc;
#[cfg(not(feature = "parking_lot"))]
use std::sync::{Condvar, Mutex};
#[cfg(feature = "triomphe")]
use triomphe::Arc;

struct Inner {
    cvar: Condvar,
    count: Mutex<usize>,
}

/// A WaitGroup waits for a collection of threads to finish.
/// The main thread calls [`add`] to set the number of
/// thread to wait for. Then each of the goroutines
/// runs and calls Done when finished. At the same time,
/// Wait can be used to block until all goroutines have finished.
///
/// A WaitGroup must not be copied after first use.
///
/// # Example
///
/// ```rust
/// use wg::WaitGroup;
/// use std::sync::Arc;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use std::time::Duration;
/// use std::thread::{spawn, sleep};
///
/// let wg = WaitGroup::new();
/// let ctr = Arc::new(AtomicUsize::new(0));
///
/// for _ in 0..5 {
///     let ctrx = ctr.clone();
///     let t_wg = wg.add(1);
///     spawn(move || {
///         // mock some time consuming task
///         sleep(Duration::from_millis(50));
///         ctrx.fetch_add(1, Ordering::Relaxed);
///
///         // mock task is finished
///         t_wg.done();
///     });
/// }
///
/// wg.wait();
/// assert_eq!(ctr.load(Ordering::Relaxed), 5);
/// ```
///
/// [`wait`]: struct.WaitGroup.html#method.wait
/// [`add`]: struct.WaitGroup.html#method.add
pub struct WaitGroup {
    inner: Arc<Inner>,
}

impl Default for WaitGroup {
    fn default() -> Self {
        Self {
            inner: Arc::new(Inner {
                cvar: Condvar::new(),
                count: Mutex::new(0),
            }),
        }
    }
}

impl From<usize> for WaitGroup {
    fn from(count: usize) -> Self {
        Self {
            inner: Arc::new(Inner {
                cvar: Condvar::new(),
                count: Mutex::new(count),
            }),
        }
    }
}

impl Clone for WaitGroup {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl std::fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let count = self.inner.count.lock_me();
        f.debug_struct("WaitGroup").field("count", &*count).finish()
    }
}

impl WaitGroup {
    /// Creates a new wait group and returns the single reference to it.
    ///
    /// # Examples
    ///
    /// ```
    /// use wg::WaitGroup;
    ///
    /// let wg = WaitGroup::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds delta to the WaitGroup counter.
    /// If the counter becomes zero, all threads blocked on [`wait`] are released.
    ///
    /// Note that calls with a delta that occur when the counter is zero
    /// must happen before a Wait.
    /// Typically this means the calls to add should execute before the statement
    /// creating the thread or other event to be waited for.
    /// If a `WaitGroup` is reused to [`wait`] for several independent sets of events,
    /// new `add` calls must happen after all previous [`wait`] calls have returned.
    ///
    /// # Example
    /// ```rust
    /// use wg::WaitGroup;
    ///
    /// let wg = WaitGroup::new();
    ///
    /// wg.add(3);
    /// (0..3).for_each(|_| {
    ///     let t_wg = wg.clone();
    ///     std::thread::spawn(move || {
    ///         // do some time consuming work
    ///         t_wg.done();
    ///     });
    /// });
    ///
    /// wg.wait();
    /// ```
    ///
    /// [`wait`]: struct.AsyncWaitGroup.html#method.wait
    pub fn add(&self, num: usize) -> Self {
        let mut ctr = self.inner.count.lock_me();

        *ctr += num;
        Self {
            inner: self.inner.clone(),
        }
    }

    /// done decrements the WaitGroup counter by one.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wg::WaitGroup;
    /// use std::thread;
    ///
    /// let wg = WaitGroup::new();
    /// wg.add(1);
    /// let t_wg = wg.clone();
    /// thread::spawn(move || {
    ///     // do some time consuming task
    ///     t_wg.done()
    /// });
    ///
    /// ```
    pub fn done(&self) {
        let mut val = self.inner.count.lock_me();

        *val = if val.eq(&1) {
            self.inner.cvar.notify_all();
            0
        } else if val.eq(&0) {
            0
        } else {
            *val - 1
        };
    }

    /// waitings return how many jobs are waiting.
    pub fn waitings(&self) -> usize {
        *self.inner.count.lock_me()
    }

    /// wait blocks until the WaitGroup counter is zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wg::WaitGroup;
    /// use std::thread;
    ///
    /// let wg = WaitGroup::new();
    /// wg.add(1);
    /// let t_wg = wg.clone();
    /// thread::spawn(move || {
    ///     // do some time consuming task
    ///     t_wg.done()
    /// });
    ///
    /// // wait other thread completes
    /// wg.wait();
    /// ```
    pub fn wait(&self) {
        let mut ctr = self.inner.count.lock_me();

        if ctr.eq(&0) {
            return;
        }

        while *ctr > 0 {
            #[cfg(feature = "parking_lot")]
            {
                self.inner.cvar.wait(&mut ctr);
            }

            #[cfg(not(feature = "parking_lot"))]
            {
                ctr = self.inner.cvar.wait(ctr).unwrap();
            }
        }
    }
}

#[cfg(feature = "future")]
pub use r#async::*;

#[cfg(feature = "future")]
mod r#async {
    use super::*;
    use event_listener::{Event, EventListener};
    use event_listener_strategy::{easy_wrapper, EventListenerFuture, Strategy};

    use std::{
        pin::Pin,
        sync::atomic::{AtomicUsize, Ordering},
        task::Poll,
    };

    #[derive(Debug)]
    struct AsyncInner {
        counter: AtomicUsize,
        event: Event,
    }

    /// An AsyncWaitGroup waits for a collection of threads to finish.
    /// The main thread calls [`add`] to set the number of
    /// thread to wait for. Then each of the goroutines
    /// runs and calls Done when finished. At the same time,
    /// Wait can be used to block until all goroutines have finished.
    ///
    /// A WaitGroup must not be copied after first use.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wg::AsyncWaitGroup;
    /// use std::sync::Arc;
    /// use std::sync::atomic::{AtomicUsize, Ordering};
    /// use tokio::{spawn, time::{sleep, Duration}};
    ///
    /// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
    /// async fn main() {
    ///     let wg = AsyncWaitGroup::new();
    ///     let ctr = Arc::new(AtomicUsize::new(0));
    ///
    ///     for _ in 0..5 {
    ///         let ctrx = ctr.clone();
    ///         let t_wg = wg.add(1);
    ///         spawn(async move {
    ///             // mock some time consuming task
    ///             sleep(Duration::from_millis(50)).await;
    ///             ctrx.fetch_add(1, Ordering::Relaxed);
    ///
    ///             // mock task is finished
    ///             t_wg.done();
    ///         });
    ///     }
    ///
    ///     wg.wait().await;
    ///     assert_eq!(ctr.load(Ordering::Relaxed), 5);
    /// }
    /// ```
    ///
    /// [`wait`]: struct.AsyncWaitGroup.html#method.wait
    /// [`add`]: struct.AsyncWaitGroup.html#method.add
    #[cfg_attr(docsrs, doc(cfg(feature = "future")))]
    pub struct AsyncWaitGroup {
        inner: Arc<AsyncInner>,
    }

    impl Default for AsyncWaitGroup {
        fn default() -> Self {
            Self {
                inner: Arc::new(AsyncInner {
                    counter: AtomicUsize::new(0),
                    event: Event::new(),
                }),
            }
        }
    }

    impl From<usize> for AsyncWaitGroup {
        fn from(count: usize) -> Self {
            Self {
                inner: Arc::new(AsyncInner {
                    counter: AtomicUsize::new(count),
                    event: Event::new(),
                }),
            }
        }
    }

    impl Clone for AsyncWaitGroup {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }

    impl std::fmt::Debug for AsyncWaitGroup {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("AsyncWaitGroup")
                .field("counter", &self.inner.counter)
                .finish()
        }
    }

    impl AsyncWaitGroup {
        /// Creates a new `AsyncWaitGroup`
        pub fn new() -> Self {
            Self::default()
        }

        /// Adds delta to the WaitGroup counter.
        /// If the counter becomes zero, all threads blocked on [`wait`] are released.
        ///
        /// Note that calls with a delta that occur when the counter is zero
        /// must happen before a Wait.
        /// Typically this means the calls to add should execute before the statement
        /// creating the thread or other event to be waited for.
        /// If a `AsyncWaitGroup` is reused to [`wait`] for several independent sets of events,
        /// new `add` calls must happen after all previous [`wait`] calls have returned.
        ///
        /// # Example
        /// ```rust
        /// use wg::AsyncWaitGroup;
        ///
        /// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
        /// async fn main() {
        ///     let wg = AsyncWaitGroup::new();
        ///
        ///     wg.add(3);
        ///     (0..3).for_each(|_| {
        ///         let t_wg = wg.clone();
        ///         tokio::spawn(async move {
        ///             // do some time consuming work
        ///             t_wg.done();
        ///         });
        ///     });
        ///
        ///     wg.wait().await;
        /// }
        /// ```
        ///
        /// [`wait`]: struct.AsyncWaitGroup.html#method.wait
        pub fn add(&self, num: usize) -> Self {
            self.inner.counter.fetch_add(num, Ordering::AcqRel);

            Self {
                inner: self.inner.clone(),
            }
        }

        /// done decrements the WaitGroup counter by one.
        ///
        /// # Example
        ///
        /// ```rust
        /// use wg::AsyncWaitGroup;
        ///
        /// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
        /// async fn main() {
        ///     let wg = AsyncWaitGroup::new();
        ///     wg.add(1);
        ///     let t_wg = wg.clone();
        ///     tokio::spawn(async move {
        ///         // do some time consuming task
        ///         t_wg.done();
        ///     });
        /// }
        /// ```
        pub fn done(&self) {
            if self.inner.counter.fetch_sub(1, Ordering::SeqCst) == 1 {
                self.inner.event.notify(usize::MAX);
            }
        }

        /// waitings return how many jobs are waiting.
        pub fn waitings(&self) -> usize {
            self.inner.counter.load(Ordering::Acquire)
        }

        /// wait blocks until the [`AsyncWaitGroup`] counter is zero.
        ///
        /// # Example
        ///
        /// ```rust
        /// use wg::AsyncWaitGroup;
        ///
        /// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
        /// async fn main() {
        ///     let wg = AsyncWaitGroup::new();
        ///     wg.add(1);
        ///     let t_wg = wg.clone();
        ///
        ///     tokio::spawn( async move {
        ///         // do some time consuming task
        ///         t_wg.done()
        ///     });
        ///
        ///     // wait other thread completes
        ///     wg.wait().await;
        /// }
        /// ```
        pub fn wait(&self) -> WaitGroupFuture<'_> {
            WaitGroupFuture::_new(WaitGroupFutureInner::new(&self.inner))
        }

        /// Wait blocks until the [`AsyncWaitGroup`] counter is zero. This method is
        /// intended to be used in a non-async context,
        /// e.g. when implementing the [`Drop`] trait.
        ///
        /// The implementation is like a spin lock, which is not efficient, so use it with caution.
        ///
        /// # Example
        ///
        /// ```rust
        /// use wg::AsyncWaitGroup;
        ///
        /// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
        /// async fn main() {
        ///     let wg = AsyncWaitGroup::new();
        ///     wg.add(1);
        ///     let t_wg = wg.clone();
        ///
        ///     tokio::spawn( async move {
        ///         // do some time consuming task
        ///         t_wg.done()
        ///     });
        ///
        ///     // wait other thread completes
        ///     wg.block_wait();
        /// }
        /// ```
        pub fn block_wait(&self) {
            WaitGroupFutureInner::new(&self.inner).wait();
        }
    }

    easy_wrapper! {
        /// A future returned by [`AsyncWaitGroup::wait()`].
        #[derive(Debug)]
        #[must_use = "futures do nothing unless you `.await` or poll them"]
        #[cfg_attr(docsrs, doc(cfg(feature = "future")))]
        pub struct WaitGroupFuture<'a>(WaitGroupFutureInner<'a> => ());

        #[cfg(all(feature = "std", not(target_family = "wasm")))]
        pub(crate) wait();
    }

    pin_project_lite::pin_project! {
        /// A future that used to wait for the [`AsyncWaitGroup`] counter is zero.
        #[must_use = "futures do nothing unless you `.await` or poll them"]
        #[project(!Unpin)]
        #[derive(Debug)]
        struct WaitGroupFutureInner<'a> {
            inner: &'a Arc<AsyncInner>,
            listener: Option<EventListener>,
            #[pin]
            _pin: std::marker::PhantomPinned,
        }
    }

    impl<'a> WaitGroupFutureInner<'a> {
        fn new(inner: &'a Arc<AsyncInner>) -> Self {
            Self {
                inner,
                listener: None,
                _pin: std::marker::PhantomPinned,
            }
        }
    }

    impl EventListenerFuture for WaitGroupFutureInner<'_> {
        type Output = ();

        fn poll_with_strategy<'a, S: Strategy<'a>>(
            self: Pin<&mut Self>,
            strategy: &mut S,
            context: &mut S::Context,
        ) -> Poll<Self::Output> {
            let this = self.project();
            loop {
                if this.inner.counter.load(Ordering::Acquire) == 0 {
                    return Poll::Ready(());
                }

                if this.listener.is_some() {
                    // Poll using the given strategy
                    match S::poll(strategy, &mut *this.listener, context) {
                        Poll::Ready(_) => {}
                        Poll::Pending => return Poll::Pending,
                    }
                } else {
                    *this.listener = Some(this.inner.event.listen());
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::time::Duration;

        #[tokio::test]
        async fn test_async_wait_group() {
            let wg = AsyncWaitGroup::new();
            let ctr = Arc::new(AtomicUsize::new(0));

            for _ in 0..5 {
                let ctrx = ctr.clone();
                let wg = wg.add(1);

                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    ctrx.fetch_add(1, Ordering::Relaxed);
                    wg.done();
                });
            }
            wg.wait().await;
            assert_eq!(ctr.load(Ordering::Relaxed), 5);
        }

        #[tokio::test]
        async fn test_async_wait_group_reuse() {
            let wg = AsyncWaitGroup::new();
            let ctr = Arc::new(AtomicUsize::new(0));
            for _ in 0..6 {
                let wg = wg.add(1);
                let ctrx = ctr.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(5)).await;
                    ctrx.fetch_add(1, Ordering::Relaxed);
                    wg.done();
                });
            }

            wg.wait().await;
            assert_eq!(ctr.load(Ordering::Relaxed), 6);

            let worker = wg.add(1);

            let ctrx = ctr.clone();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(5)).await;
                ctrx.fetch_add(1, Ordering::Relaxed);
                worker.done();
            });

            wg.wait().await;
            assert_eq!(ctr.load(Ordering::Relaxed), 7);
        }

        #[tokio::test]
        async fn test_async_wait_group_nested() {
            let wg = AsyncWaitGroup::new();
            let ctr = Arc::new(AtomicUsize::new(0));
            for _ in 0..5 {
                let worker = wg.add(1);
                let ctrx = ctr.clone();
                tokio::spawn(async move {
                    let nested_worker = worker.add(1);
                    let ctrxx = ctrx.clone();
                    tokio::spawn(async move {
                        ctrxx.fetch_add(1, Ordering::Relaxed);
                        nested_worker.done();
                    });
                    ctrx.fetch_add(1, Ordering::Relaxed);
                    worker.done();
                });
            }

            wg.wait().await;
            assert_eq!(ctr.load(Ordering::Relaxed), 10);
        }

        #[tokio::test]
        async fn test_async_wait_group_from() {
            let wg = AsyncWaitGroup::from(5);
            for _ in 0..5 {
                let t = wg.clone();
                tokio::spawn(async move {
                    t.done();
                });
            }
            wg.wait().await;
        }

        #[test]
        fn test_sync_wait_group() {
            let wg = WaitGroup::new();
            let ctr = Arc::new(AtomicUsize::new(0));

            for _ in 0..5 {
                let ctrx = ctr.clone();
                let wg = wg.add(1);
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_millis(50));
                    ctrx.fetch_add(1, Ordering::Relaxed);

                    wg.done();
                });
            }
            wg.wait();
            assert_eq!(ctr.load(Ordering::Relaxed), 5);
        }

        #[tokio::test]
        async fn test_async_waitings() {
            let wg = AsyncWaitGroup::new();
            wg.add(1);
            wg.add(1);
            assert_eq!(wg.waitings(), 2);
        }

        #[test]
        fn test_async_block_wait() {
            let wg = AsyncWaitGroup::new();
            let t_wg = wg.add(1);
            std::thread::spawn(move || {
                // do some time consuming task
                t_wg.done();
            });

            // wait other thread completes
            wg.block_wait();

            assert_eq!(wg.waitings(), 0);
        }

        #[async_std::test]
        async fn test_wake_after_updating() {
            let wg = AsyncWaitGroup::new();
            for _ in 0..100000 {
                let worker = wg.add(1);
                async_std::task::spawn(async move {
                    async_std::task::sleep(std::time::Duration::from_millis(10)).await;
                    let mut a = 0;
                    for _ in 0..1000 {
                        a += 1;
                    }
                    println!("{a}");
                    async_std::task::sleep(std::time::Duration::from_millis(10)).await;
                    worker.done();
                });
            }
            wg.wait().await;
        }

        #[test]
        fn test_clone_and_fmt() {
            let awg = AsyncWaitGroup::new();
            let awg1 = awg.clone();
            awg1.add(3);
            assert_eq!(format!("{:?}", awg), format!("{:?}", awg1));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_sync_wait_group_reuse() {
        let wg = WaitGroup::new();
        let ctr = Arc::new(AtomicUsize::new(0));
        for _ in 0..6 {
            let wg = wg.add(1);
            let ctrx = ctr.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(5));
                ctrx.fetch_add(1, Ordering::Relaxed);
                wg.done();
            });
        }

        wg.wait();
        assert_eq!(ctr.load(Ordering::Relaxed), 6);

        let worker = wg.add(1);
        let ctrx = ctr.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(5));
            ctrx.fetch_add(1, Ordering::Relaxed);
            worker.done();
        });
        wg.wait();
        assert_eq!(ctr.load(Ordering::Relaxed), 7);
    }

    #[test]
    fn test_sync_wait_group_nested() {
        let wg = WaitGroup::new();
        let ctr = Arc::new(AtomicUsize::new(0));
        for _ in 0..5 {
            let worker = wg.add(1);
            let ctrx = ctr.clone();
            std::thread::spawn(move || {
                let nested_worker = worker.add(1);
                let ctrxx = ctrx.clone();
                std::thread::spawn(move || {
                    ctrxx.fetch_add(1, Ordering::Relaxed);
                    nested_worker.done();
                });
                ctrx.fetch_add(1, Ordering::Relaxed);
                worker.done();
            });
        }

        wg.wait();
        assert_eq!(ctr.load(Ordering::Relaxed), 10);
    }

    #[test]
    fn test_sync_wait_group_from() {
        std::thread::scope(|s| {
            let wg = WaitGroup::from(5);
            for _ in 0..5 {
                let t = wg.clone();
                s.spawn(move || {
                    t.done();
                });
            }
            wg.wait();
        });
    }

    #[test]
    fn test_clone_and_fmt() {
        let swg = WaitGroup::new();
        let swg1 = swg.clone();
        swg1.add(3);
        assert_eq!(format!("{:?}", swg), format!("{:?}", swg1));
    }

    #[test]
    fn test_waitings() {
        let wg = WaitGroup::new();
        wg.add(1);
        wg.add(1);
        assert_eq!(wg.waitings(), 2);
    }
}
