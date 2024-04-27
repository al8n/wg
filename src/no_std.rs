use core::sync::atomic::{AtomicUsize, Ordering};

use alloc::sync::Arc;

#[derive(Debug)]
struct Inner {
    counter: AtomicUsize,
}

/// An WaitGroup waits for a collection of threads to finish.
/// The main thread calls [`add`] to set the number of
/// thread to wait for. Then each of the tasks
/// runs and calls [`WaitGroup::done`](WaitGroup::done) when finished. At the same time,
/// Wait can be used to block until all tasks have finished.
///
/// [`wait`]: struct.WaitGroup.html#method.wait
/// [`add`]: struct.WaitGroup.html#method.add
#[cfg_attr(docsrs, doc(cfg(feature = "future")))]
pub struct WaitGroup {
    inner: Arc<Inner>,
}

impl Default for WaitGroup {
    fn default() -> Self {
        Self {
            inner: Arc::new(Inner {
                counter: AtomicUsize::new(0),
            }),
        }
    }
}

impl From<usize> for WaitGroup {
    fn from(count: usize) -> Self {
        Self {
            inner: Arc::new(Inner {
                counter: AtomicUsize::new(count),
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

impl WaitGroup {
    /// Creates a new `WaitGroup`
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
    ///
    /// ```rust,ignore
    /// use wg::WaitGroup;
    ///
    /// let wg = WaitGroup::new();
    ///
    /// wg.add(3);
    /// (0..3).for_each(|_| {
    ///     let t_wg = wg.clone();
    ///     spawn(move || {
    ///         // do some time consuming work
    ///         t_wg.done();
    ///     });
    /// });
    ///
    /// wg.wait();
    /// ```
    ///
    /// [`wait`]: struct.WaitGroup.html#method.wait
    pub fn add(&self, num: usize) -> Self {
        self.inner.counter.fetch_add(num, Ordering::AcqRel);

        Self {
            inner: self.inner.clone(),
        }
    }

    /// Decrements the WaitGroup counter by one, returning the remaining count.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use wg::WaitGroup;
    ///
    /// let wg = WaitGroup::new();
    /// wg.add(1);
    /// let t_wg = wg.clone();
    /// spawn(move || {
    ///     // do some time consuming task
    ///     t_wg.done();
    /// });
    /// ```
    pub fn done(&self) -> usize {
        match self
            .inner
            .counter
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| {
                if v != 0 {
                    Some(v - 1)
                } else {
                    None
                }
            }) {
            Ok(x) => x,
            Err(x) => {
                assert_eq!(x, 0);
                x
            }
        }
    }

    /// waitings return how many jobs are waiting.
    pub fn waitings(&self) -> usize {
        self.inner.counter.load(Ordering::Acquire)
    }

    /// wait blocks until the [`WaitGroup`] counter is zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use wg::WaitGroup;
    ///
    /// let wg = WaitGroup::new();
    /// wg.add(1);
    /// let t_wg = wg.clone();
    ///
    /// spawn(async move {
    ///     // do some time consuming task
    ///     t_wg.done()
    /// });
    ///
    /// // wait other thread completes
    /// wg.wait();
    /// ```
    pub fn wait(&self) {
        while self.inner.counter.load(Ordering::SeqCst) != 0 {
            core::hint::spin_loop();
        }
    }
}
