use super::*;
use ::tokio::sync::{futures::Notified, Notify};

use std::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
    task::{Context, Poll},
};

#[derive(Debug)]
struct AsyncInner {
    counter: AtomicUsize,
    notify: Notify,
}

/// An AsyncWaitGroup waits for a collection of threads to finish.
/// The main thread calls [`add`] to set the number of
/// thread to wait for. Then each of the tasks
/// runs and calls Done when finished. At the same time,
/// Wait can be used to block until all tasks have finished.
///
/// A WaitGroup must not be copied after first use.
///
/// # Example
///
/// ```rust
/// use wg::tokio::AsyncWaitGroup;
/// use std::sync::Arc;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use tokio::{spawn, time::{sleep, Duration}};
///
/// #[tokio::main]
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
                counter: AtomicUsize::new(0),
                notify: Notify::new(),
            }),
        }
    }
}

impl From<usize> for AsyncWaitGroup {
    fn from(count: usize) -> Self {
        Self {
            inner: Arc::new(AsyncInner {
                counter: AtomicUsize::new(count),
                notify: Notify::new(),
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
    /// use wg::tokio::AsyncWaitGroup;
    ///
    /// #[tokio::main]
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
    /// use wg::tokio::AsyncWaitGroup;
    ///
    /// #[tokio::main]
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
    pub fn done(self) {
        if self.inner.counter.fetch_sub(1, Ordering::SeqCst) == 1 {
            self.inner.notify.notify_waiters();
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
    /// use wg::tokio::AsyncWaitGroup;
    ///
    /// #[tokio::main]
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
        WaitGroupFuture {
            inner: self,
            notified: self.inner.notify.notified(),
            _pin: std::marker::PhantomPinned,
        }
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
    /// use wg::tokio::AsyncWaitGroup;
    ///
    /// #[tokio::main(flavor = "multi_thread")]
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
        let this = self.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        ::tokio::task::spawn(async move {
            this.wait().await;
            let _ = tx.send(());
        });
        let _ = rx.recv();
    }
}

pin_project_lite::pin_project! {
    /// A future returned by [`AsyncWaitGroup::wait()`].
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
    pub struct WaitGroupFuture<'a> {
        inner: &'a AsyncWaitGroup,
        #[pin]
        notified: Notified<'a>,
        #[pin]
        _pin: std::marker::PhantomPinned,
    }
}

impl<'a> Future for WaitGroupFuture<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.inner.inner.counter.load(Ordering::SeqCst) == 0 {
            return Poll::Ready(());
        }

        let this = self.project();
        match this.notified.poll(cx) {
            Poll::Pending => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Poll::Ready(_) => {
                if this.inner.inner.counter.load(Ordering::SeqCst) == 0 {
                    Poll::Ready(())
                } else {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        }
    }
}
