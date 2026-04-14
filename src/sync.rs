trait Mu {
    type Guard<'a>
    where
        Self: 'a;
    fn lock_me(&self) -> Self::Guard<'_>;
}

#[cfg(feature = "parking_lot")]
impl<T: ?Sized> Mu for parking_lot::Mutex<T> {
    type Guard<'a>
        = parking_lot::MutexGuard<'a, T>
    where
        Self: 'a;

    fn lock_me(&self) -> Self::Guard<'_> {
        self.lock()
    }
}

#[cfg(not(feature = "parking_lot"))]
impl<T: ?Sized> Mu for std::sync::Mutex<T> {
    type Guard<'a>
        = std::sync::MutexGuard<'a, T>
    where
        Self: 'a;

    fn lock_me(&self) -> Self::Guard<'_> {
        // Poisoning is not meaningful for a `usize` counter: the worst a
        // panicking thread can leave behind is a stale count, not corrupt
        // memory. Recovering the guard avoids cascading panics across all
        // other threads that touch this WaitGroup.
        self.lock().unwrap_or_else(|e| e.into_inner())
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
///
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

/// Shorthand for [`add`](WaitGroup::add), discarding the returned clone.
///
/// ```
/// use wg::WaitGroup;
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

    /// Increments the counter by `num` and returns a handle sharing the
    /// same counter.
    ///
    /// The returned value is a clone of the existing `WaitGroup`, **not**
    /// a separate group — all copies operate on the same underlying
    /// counter. Returning the handle is a convenience for the common
    /// pattern of immediately passing ownership to a spawned task:
    ///
    /// ```rust
    /// use wg::WaitGroup;
    ///
    /// let wg = WaitGroup::new();
    /// let t_wg = wg.add(1);
    /// std::thread::spawn(move || {
    ///     // do some time consuming work
    ///     t_wg.done();
    /// });
    /// wg.wait();
    /// ```
    ///
    /// Ignoring the return value is valid — it just drops a cheap handle;
    /// the counter increment is still in effect. Use this form when you
    /// want to spawn multiple workers that each clone the group:
    ///
    /// ```rust
    /// use wg::WaitGroup;
    ///
    /// let wg = WaitGroup::new();
    /// wg.add(3);                         // counter += 3
    /// for _ in 0..3 {
    ///     let t_wg = wg.clone();         // clone for each task
    ///     std::thread::spawn(move || {
    ///         t_wg.done();
    ///     });
    /// }
    /// wg.wait();
    /// ```
    ///
    /// When the counter later reaches zero, all threads blocked in
    /// [`wait`](Self::wait) are released.
    ///
    /// # Ordering requirements
    ///
    /// Calls that bring the counter up from zero must happen before any
    /// [`wait`](Self::wait) call — typically by running them on the main
    /// thread before spawning the workers.
    ///
    /// If a `WaitGroup` is reused for several independent rounds, new
    /// `add` calls must happen after all previous [`wait`](Self::wait)
    /// calls have returned.
    pub fn add(&self, num: usize) -> Self {
        let mut ctr = self.inner.count.lock_me();
        // In debug builds, give a clear message on overflow. `+=` already
        // panics in debug builds on overflow, but with a generic message.
        debug_assert!(
            ctr.checked_add(num).is_some(),
            "WaitGroup counter overflow: {ctr} + {num}",
            ctr = *ctr,
        );
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

    /// Returns the current counter value — the number of threads still
    /// waiting to complete.
    pub fn remaining(&self) -> usize {
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
                ctr = self.inner.cvar.wait(ctr).unwrap_or_else(|e| e.into_inner());
            }
        }
    }
}
