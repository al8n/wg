use event_listener::{Event, EventListener};

use core::{
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
    task::{Context, Poll},
};

#[cfg(feature = "triomphe")]
use triomphe::Arc;

#[cfg(all(feature = "std", not(feature = "triomphe")))]
use std::sync::Arc;

#[cfg(all(not(feature = "std"), not(feature = "triomphe")))]
use alloc::sync::Arc;

#[derive(Debug)]
struct AsyncInner {
    counter: AtomicUsize,
    event: Event,
}

/// An WaitGroup waits for a collection of threads to finish.
///
/// The main thread calls [`add`] to set the number of
/// thread to wait for. Then each of the tasks
/// runs and calls Done when finished. At the same time,
/// Wait can be used to block until all tasks have finished.
///
/// # Example
///
/// ```rust
/// use wg::future::WaitGroup;
/// use std::sync::Arc;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use std::time::Duration;
/// use tokio::time::sleep;
///
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// let wg = WaitGroup::new();
/// let ctr = Arc::new(AtomicUsize::new(0));
///
/// for _ in 0..5 {
///     let ctrx = ctr.clone();
///     let t_wg = wg.add(1);
///     tokio::spawn(async move {
///         // mock some time consuming task
///         sleep(Duration::from_millis(50)).await;
///         ctrx.fetch_add(1, Ordering::Relaxed);
///
///         // mock task is finished
///         t_wg.done();
///     });
/// }
///
/// wg.wait().await;
/// assert_eq!(ctr.load(Ordering::Relaxed), 5);
/// # })
/// ```
///
/// [`wait`]: struct.WaitGroup.html#method.wait
/// [`add`]: struct.WaitGroup.html#method.add
#[cfg_attr(docsrs, doc(cfg(feature = "future")))]
pub struct WaitGroup {
    inner: Arc<AsyncInner>,
}

impl Default for WaitGroup {
    fn default() -> Self {
        Self {
            inner: Arc::new(AsyncInner {
                counter: AtomicUsize::new(0),
                event: Event::new(),
            }),
        }
    }
}

impl From<usize> for WaitGroup {
    fn from(count: usize) -> Self {
        Self {
            inner: Arc::new(AsyncInner {
                counter: AtomicUsize::new(count),
                event: Event::new(),
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

impl core::fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WaitGroup")
            .field("counter", &self.inner.counter)
            .finish()
    }
}

/// Shorthand for [`add`](WaitGroup::add), discarding the returned clone.
///
/// ```
/// use wg::future::WaitGroup;
/// let mut wg = WaitGroup::new();
/// wg += 3;
/// assert_eq!(wg.remaining(), 3);
/// ```
impl core::ops::AddAssign<usize> for WaitGroup {
    fn add_assign(&mut self, rhs: usize) {
        self.add(rhs);
    }
}

impl WaitGroup {
    /// Creates a new `WaitGroup`
    pub fn new() -> Self {
        Self::default()
    }

    /// Increments the counter by `num` and returns a handle sharing the
    /// same counter.
    ///
    /// The returned value is a clone of the existing `WaitGroup`, **not**
    /// a separate group — all copies operate on the same underlying
    /// counter. Returning the handle is a convenience for the common
    /// pattern of immediately passing ownership to a spawned task:
    ///
    /// ```rust
    /// use wg::future::WaitGroup;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let wg = WaitGroup::new();
    /// let t_wg = wg.add(1);
    /// tokio::spawn(async move {
    ///     // do some time consuming work
    ///     t_wg.done();
    /// });
    /// wg.wait().await;
    /// # })
    /// ```
    ///
    /// Ignoring the return value is valid — it just drops a cheap handle;
    /// the counter increment is still in effect. Use this form when you
    /// want to spawn multiple workers that each clone the group:
    ///
    /// ```rust
    /// use wg::future::WaitGroup;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let wg = WaitGroup::new();
    /// wg.add(3);                         // counter += 3
    /// for _ in 0..3 {
    ///     let t_wg = wg.clone();         // clone for each task
    ///     tokio::spawn(async move {
    ///         t_wg.done();
    ///     });
    /// }
    /// wg.wait().await;
    /// # })
    /// ```
    ///
    /// When the counter later reaches zero, all tasks blocked in
    /// [`wait`](Self::wait) are released.
    ///
    /// # Ordering requirements
    ///
    /// Calls that bring the counter up from zero must happen before any
    /// [`wait`](Self::wait) call — typically by running them on the main
    /// task before spawning the workers.
    ///
    /// If a `WaitGroup` is reused for several independent rounds, new
    /// `add` calls must happen after all previous [`wait`](Self::wait)
    /// calls have returned.
    pub fn add(&self, num: usize) -> Self {
        let prev = self.inner.counter.fetch_add(num, Ordering::Release);
        // In debug builds, catch the (essentially unreachable) case where the
        // counter wraps past `usize::MAX`. Silent wrap in release.
        debug_assert!(
            prev.checked_add(num).is_some(),
            "WaitGroup counter overflow: {prev} + {num}"
        );

        Self {
            inner: self.inner.clone(),
        }
    }

    /// Decrements the `WaitGroup` counter by one and returns the
    /// remaining count.
    ///
    /// If the counter is already zero, this call is a no-op and returns `0`.
    /// No panic is raised.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wg::future::WaitGroup;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    ///     let wg = WaitGroup::new();
    ///     wg.add(1);
    ///     let t_wg = wg.clone();
    ///     tokio::spawn(async move {
    ///         // do some time consuming task
    ///         t_wg.done();
    ///     });
    /// # })
    /// ```
    pub fn done(&self) -> usize {
        match self
            .inner
            .counter
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |v| v.checked_sub(1))
        {
            Ok(old) => {
                let remaining = old - 1;
                // Only notify when the counter actually reaches zero. Waking
                // listeners on every decrement just makes them re-check and
                // sleep again, wasting work.
                if remaining == 0 {
                    self.inner.event.notify(usize::MAX);
                }
                remaining
            }
            // Over-done: counter was already zero. Silently no-op.
            Err(_) => 0,
        }
    }

    /// Returns the current counter value — the number of tasks still
    /// waiting to complete.
    pub fn remaining(&self) -> usize {
        self.inner.counter.load(Ordering::Acquire)
    }

    /// wait blocks until the [`WaitGroup`] counter is zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wg::future::WaitGroup;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let wg = WaitGroup::new();
    /// wg.add(1);
    /// let t_wg = wg.clone();
    ///
    /// tokio::spawn(async move {
    ///     // do some time consuming task
    ///     t_wg.done()
    /// });
    ///
    /// // wait other thread completes
    /// wg.wait().await;
    /// # })
    /// ```
    pub fn wait(&self) -> WaitGroupFuture<'_> {
        WaitGroupFuture {
            inner: self,
            notified: self.inner.event.listen(),
            _pin: core::marker::PhantomPinned,
        }
    }

    /// Wait blocks until the [`WaitGroup`] counter is zero. This method is
    /// intended to be used in a non-async context,
    /// e.g. when implementing the [`Drop`] trait.
    ///
    /// The implementation is like a spin lock, which is not efficient, so use it with caution.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wg::future::WaitGroup;
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let wg = WaitGroup::new();
    /// wg.add(1);
    /// let t_wg = wg.clone();
    ///
    /// tokio::spawn(async move {
    ///     // do some time consuming task
    ///     t_wg.done()
    /// });
    ///
    /// // wait other thread completes
    /// wg.wait_blocking();
    /// # })
    /// ```
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn wait_blocking(&self) {
        use event_listener::Listener;

        while self.inner.counter.load(Ordering::Acquire) != 0 {
            let ln = self.inner.event.listen();
            // Re-check after creating the listener to close the lost-wakeup
            // window: if `done()` already notified before we listened, the
            // counter is now zero and we can bail out.
            if self.inner.counter.load(Ordering::Acquire) == 0 {
                return;
            }
            ln.wait();
        }
    }
}

pin_project_lite::pin_project! {
    /// A future returned by [`WaitGroup::wait()`].
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct WaitGroupFuture<'a> {
        inner: &'a WaitGroup,
        #[pin]
        notified: EventListener,
        #[pin]
        _pin: core::marker::PhantomPinned,
    }
}

impl core::future::Future for WaitGroupFuture<'_> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.inner.inner.counter.load(Ordering::Acquire) == 0 {
            return Poll::Ready(());
        }

        let mut this = self.project();
        match this.notified.as_mut().poll(cx) {
            Poll::Pending => {
                // The listener has registered our waker. Re-check the counter
                // to close the lost-wakeup window (if `done()` notified
                // before we polled the listener, the counter is now zero).
                //
                // Do NOT call `wake_by_ref` here — the listener will wake us
                // when notified. Calling `wake_by_ref` would cause a busy
                // re-poll loop that never yields to the executor.
                if this.inner.inner.counter.load(Ordering::Acquire) == 0 {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
            Poll::Ready(_) => {
                // The previous listener was notified. Check whether we're
                // actually done; if not, register a fresh listener.
                if this.inner.inner.counter.load(Ordering::Acquire) == 0 {
                    Poll::Ready(())
                } else {
                    // Install a new listener and poll it to register our
                    // waker inline. If it is already notified (raced with
                    // another `done`), re-check the counter; if still
                    // non-zero, fall back to `wake_by_ref` for another pass.
                    *this.notified = this.inner.inner.event.listen();
                    match this.notified.as_mut().poll(cx) {
                        Poll::Pending => {
                            if this.inner.inner.counter.load(Ordering::Acquire) == 0 {
                                Poll::Ready(())
                            } else {
                                Poll::Pending
                            }
                        }
                        Poll::Ready(_) => {
                            if this.inner.inner.counter.load(Ordering::Acquire) == 0 {
                                Poll::Ready(())
                            } else {
                                cx.waker().wake_by_ref();
                                Poll::Pending
                            }
                        }
                    }
                }
            }
        }
    }
}
