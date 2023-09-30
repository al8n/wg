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
//! [<img alt="rustc" src="https://img.shields.io/badge/MSRV-1.56.0-fc8d62.svg?style=for-the-badge&logo=Rust" height="22">][rustc-url]
//!
//! [<img alt="license-apache" src="https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=for-the-badge&logo=Apache" height="22">][license-apache-url]
//! [<img alt="license-mit" src="https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge&fontColor=white&logoColor=f5c076&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGhlaWdodD0iMzZweCIgdmlld0JveD0iMCAwIDI0IDI0IiB3aWR0aD0iMzZweCIgZmlsbD0iI2Y1YzA3NiI+PHBhdGggZD0iTTAgMGgyNHYyNEgwVjB6IiBmaWxsPSJub25lIi8+PHBhdGggZD0iTTEwLjA4IDEwLjg2Yy4wNS0uMzMuMTYtLjYyLjMtLjg3cy4zNC0uNDYuNTktLjYyYy4yNC0uMTUuNTQtLjIyLjkxLS4yMy4yMy4wMS40NC4wNS42My4xMy4yLjA5LjM4LjIxLjUyLjM2cy4yNS4zMy4zNC41My4xMy40Mi4xNC42NGgxLjc5Yy0uMDItLjQ3LS4xMS0uOS0uMjgtMS4yOXMtLjQtLjczLS43LTEuMDEtLjY2LS41LTEuMDgtLjY2LS44OC0uMjMtMS4zOS0uMjNjLS42NSAwLTEuMjIuMTEtMS43LjM0cy0uODguNTMtMS4yLjkyLS41Ni44NC0uNzEgMS4zNlM4IDExLjI5IDggMTEuODd2LjI3YzAgLjU4LjA4IDEuMTIuMjMgMS42NHMuMzkuOTcuNzEgMS4zNS43Mi42OSAxLjIuOTFjLjQ4LjIyIDEuMDUuMzQgMS43LjM0LjQ3IDAgLjkxLS4wOCAxLjMyLS4yM3MuNzctLjM2IDEuMDgtLjYzLjU2LS41OC43NC0uOTQuMjktLjc0LjMtMS4xNWgtMS43OWMtLjAxLjIxLS4wNi40LS4xNS41OHMtLjIxLjMzLS4zNi40Ni0uMzIuMjMtLjUyLjNjLS4xOS4wNy0uMzkuMDktLjYuMS0uMzYtLjAxLS42Ni0uMDgtLjg5LS4yMy0uMjUtLjE2LS40NS0uMzctLjU5LS42MnMtLjI1LS41NS0uMy0uODgtLjA4LS42Ny0uMDgtMXYtLjI3YzAtLjM1LjAzLS42OC4wOC0xLjAxek0xMiAyQzYuNDggMiAyIDYuNDggMiAxMnM0LjQ4IDEwIDEwIDEwIDEwLTQuNDggMTAtMTBTMTcuNTIgMiAxMiAyem0wIDE4Yy00LjQxIDAtOC0zLjU5LTgtOHMzLjU5LTggOC04IDggMy41OSA4IDgtMy41OSA4LTggOHoiLz48L3N2Zz4=" height="22">][license-mit-url]
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
#![allow(clippy::needless_late_init)]

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

#[cfg(feature = "atomic-waker")]
pub use r#async::*;

#[cfg(feature = "atomic-waker")]
mod r#async {
    use super::*;
    use atomic_waker::AtomicWaker;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    };

    struct AsyncInner {
        waker: AtomicWaker,
        count: AtomicUsize,
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
    pub struct AsyncWaitGroup {
        inner: Arc<AsyncInner>,
    }

    impl Default for AsyncWaitGroup {
        fn default() -> Self {
            Self {
                inner: Arc::new(AsyncInner {
                    count: AtomicUsize::new(0),
                    waker: AtomicWaker::new(),
                }),
            }
        }
    }

    impl From<usize> for AsyncWaitGroup {
        fn from(count: usize) -> Self {
            Self {
                inner: Arc::new(AsyncInner {
                    count: AtomicUsize::new(count),
                    waker: AtomicWaker::new(),
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
            let count = self.inner.count.load(Ordering::Relaxed);

            f.debug_struct("AsyncWaitGroup")
                .field("count", &count)
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
            self.inner.count.fetch_add(num, Ordering::SeqCst);

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
            let res = self
                .inner
                .count
                .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |val| {
                    // We are the last worker
                    if val == 1 {
                        Some(0)
                    } else if val == 0 {
                        None
                    } else {
                        Some(val - 1)
                    }
                });
            if let Ok(count) = res {
                if count == 1 {
                    self.inner.waker.wake();
                }
            }
        }

        /// waitings return how many jobs are waiting.
        pub fn waitings(&self) -> usize {
            self.inner.count.load(Ordering::Acquire)
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
        pub async fn wait(&self) {
            WaitGroupFuture::new(&self.inner).await
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
            loop {
                match self.inner.count.load(Ordering::Acquire) {
                    0 => return,
                    _ => core::hint::spin_loop(),
                }
            }
        }
    }

    struct WaitGroupFuture<'a> {
        inner: &'a Arc<AsyncInner>,
    }

    impl<'a> WaitGroupFuture<'a> {
        fn new(inner: &'a Arc<AsyncInner>) -> Self {
            Self { inner }
        }
    }

    impl Future for WaitGroupFuture<'_> {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            match self.inner.count.load(Ordering::Acquire) {
                0 => Poll::Ready(()),
                _ => {
                    self.inner.waker.register(cx.waker());
                    Poll::Pending
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

        #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
        async fn test_async_block_wait() {
            let wg = AsyncWaitGroup::new();
            wg.add(1);
            let t_wg = wg.clone();

            tokio::spawn(async move {
                // do some time consuming task
                t_wg.done()
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
                    for _ in 0..1000 {
                        async_std::task::yield_now().await;
                    }
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
