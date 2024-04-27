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

    /// Decrements the WaitGroup counter by one, returning the remaining count.
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
    pub fn done(&self) -> usize {
        let mut val = self.inner.count.lock_me();

        *val = if val.eq(&1) {
            self.inner.cvar.notify_all();
            0
        } else if val.eq(&0) {
            0
        } else {
            *val - 1
        };
        *val
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
