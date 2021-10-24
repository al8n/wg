//!
#![deny(missing_docs)]

macro_rules! cfg_std {
    ($($item: item)*) => {
        $(
        #[cfg(feature = "std")]
        #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
        $item
        )*
    };
}

macro_rules! cfg_std_expr {
    ($($item: expr;)*) => {
        $(
        #[cfg(feature = "std")]
        $item
        )*
    };
}

macro_rules! cfg_not_std {
    ($($item: item)*) => {
        $(
        #[cfg(not(feature = "std"))]
        #[cfg_attr(docsrs, doc(cfg(not(feature = "std"))))]
        $item
        )*
    };
}

macro_rules! cfg_not_std_expr {
    ($($item: expr;)*) => {
        $(
        #[cfg(not(feature = "std"))]
        $item
        )*
    };
}

cfg_std! {
    use std::sync::{Mutex, Condvar};
}

cfg_not_std! {
    use parking_lot::{Condvar, Mutex};
}

use std::future::Future;
use std::ops::Sub;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Waker};

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

impl Clone for WaitGroup {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl std::fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "std")]
            let count = self.inner.count.lock().unwrap();
        #[cfg(not(feature = "std"))]
            let count = self.inner.count.lock();
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
        #[cfg(feature = "std")]
        let mut ctr = self.inner.count.lock().unwrap();
        #[cfg(not(feature = "std"))]
        let mut ctr = self.inner.count.lock();

        *ctr += num;
        Self {
            inner: self.inner.clone()
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
        #[cfg(feature = "std")]
        let mut val = self.inner.count.lock().unwrap();
        #[cfg(not(feature = "std"))]
        let mut val = self.inner.count.lock();

        *val = if val.eq(&1) {
            self.inner.cvar.notify_all();
            0
        } else if val.eq(&0) {
            0
        } else {
            val.sub(1)
        };
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
        let mut ctr;
        cfg_std_expr!(
            ctr = self.inner.count.lock().unwrap();
        );

        cfg_not_std_expr!(
            ctr = self.inner.count.lock();
        );

        if ctr.eq(&0) {
            return;
        }

        while *ctr > 0 {
            cfg_std_expr!(
                ctr = self.inner.cvar.wait(ctr).unwrap();
            );

            cfg_not_std_expr!(
                self.inner.cvar.wait(&mut ctr);
            );
        }
    }
}


struct AsyncInner {
    waker: Mutex<Option<Waker>>,
    count: AtomicUsize,
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
                waker: Mutex::new(None),
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

    cfg_not_std! {
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
        ///         t_wg.done()
        ///     });
        /// }
        /// ```
        pub fn done(&self) {
            let _ = self
                .inner
                .count
                .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |val| {
                    // We are the last worker
                    if val == 1 {
                        if let Some(waker) = self.inner.waker.lock().take() {
                            waker.wake();
                        }
                        Some(0)
                    } else if val == 0 {
                        None
                    } else {
                        Some(val - 1)
                    }
                });
        }
    }

    cfg_std! {
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
            let _ = self
                .inner
                .count
                .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |val| {
                    // We are the last worker
                    if val == 1 {
                        if let Some(waker) = self.inner.waker.lock().unwrap().take() {
                            waker.wake();
                        }
                        Some(0)
                    } else if val == 0 {
                        None
                    } else {
                        Some(val - 1)
                    }
                });
        }
    }


    /// wait blocks until the WaitGroup counter is zero.
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

    #[cfg(feature = "std")]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = cx.waker().clone();

        let mut g = self.inner.waker.lock().unwrap();
        *g = Some(waker);

        match self.inner.count.load(Ordering::Relaxed) {
            0 => Poll::Ready(()),
            _ => Poll::Pending,
        }
    }

    #[cfg(not(feature = "std"))]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let waker = cx.waker().clone();
        *self.inner.waker.lock() = Some(waker);

        match self.inner.count.load(Ordering::Relaxed) {
            0 => Poll::Ready(()),
            _ => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
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
}
