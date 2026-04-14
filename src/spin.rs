//! A lock-free, atomic-counter WaitGroup.
//!
//! [`WaitGroup`] uses atomic operations and an adaptive spin-loop to wait
//! for completion, so it needs no `Mutex`/`Condvar`. It works under `std`
//! and in `no_std + alloc` environments, but it is not usable in pure
//! `core`/no-allocation environments because it stores shared state in `Arc`.
//! On `std` the adaptive backoff yields the OS thread once its short-spin
//! budget is exhausted; on `no_std + alloc` it keeps spinning.
//!
//! Use `WaitGroup` when:
//! - You are in a `no_std + alloc` environment.
//! - The expected wait is short and you want to avoid OS synchronization
//!   overhead.
//!
//! Prefer [`WaitGroup`](crate::WaitGroup) for longer waits under `std`.
//! Prefer [`future::WaitGroup`](crate::future::WaitGroup) for async contexts.

use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(all(any(feature = "std", feature = "alloc"), not(feature = "triomphe")))]
use std::sync::Arc;

#[cfg(feature = "triomphe")]
use triomphe::Arc;

/// Adaptive backoff: spins with exponentially increasing delay, then yields
/// (on `std`) or continues spinning (on pure `no_std`).
///
/// Inspired by `crossbeam_utils::Backoff` but inlined to avoid a dependency.
#[inline]
fn backoff_step(iter: &mut u32) {
  const SPIN_LIMIT: u32 = 6;
  if *iter <= SPIN_LIMIT {
    for _ in 0..(1u32 << *iter) {
      core::hint::spin_loop();
    }
  } else {
    #[cfg(feature = "std")]
    std::thread::yield_now();
    #[cfg(not(feature = "std"))]
    for _ in 0..(1u32 << SPIN_LIMIT) {
      core::hint::spin_loop();
    }
  }
  *iter = iter.saturating_add(1);
}

#[derive(Debug)]
struct Inner {
  counter: AtomicUsize,
}

/// A lock-free WaitGroup that waits for a collection of tasks to finish.
///
/// The main thread calls [`add`](WaitGroup::add) to set the number of
/// tasks to wait for. Each task runs and calls [`done`](WaitGroup::done)
/// when finished. [`wait`](WaitGroup::wait) blocks (spinning) until all
/// tasks have finished.
///
/// # Example
///
/// ```rust
/// use wg::spin::WaitGroup;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use std::sync::Arc;
///
/// let wg = WaitGroup::new();
/// let ctr = Arc::new(AtomicUsize::new(0));
///
/// for _ in 0..5 {
///     let ctrx = ctr.clone();
///     let t_wg = wg.add(1);
///     std::thread::spawn(move || {
///         ctrx.fetch_add(1, Ordering::Relaxed);
///         t_wg.done();
///     });
/// }
///
/// wg.wait();
/// assert_eq!(ctr.load(Ordering::Relaxed), 5);
/// ```
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

/// Shorthand for [`add`](WaitGroup::add), discarding the returned clone.
///
/// ```
/// use wg::spin::WaitGroup;
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
  /// Creates a new `WaitGroup` with a counter of zero.
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
  /// use wg::spin::WaitGroup;
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
  /// use wg::spin::WaitGroup;
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
  /// thread before spawning the workers. Adding while another thread
  /// is in `wait` is a race and not supported.
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

  /// Decrements the counter by one and returns the remaining count.
  ///
  /// If the counter is already zero, this call is a no-op and returns `0`.
  /// No panic is raised.
  pub fn done(&self) -> usize {
    match self
      .inner
      .counter
      .fetch_update(Ordering::AcqRel, Ordering::Acquire, |v| v.checked_sub(1))
    {
      Ok(old) => old - 1,
      // Over-done: counter was already zero. Silently no-op.
      Err(_) => 0,
    }
  }

  /// Returns the current counter value — the number of tasks still
  /// waiting to complete.
  pub fn remaining(&self) -> usize {
    self.inner.counter.load(Ordering::Acquire)
  }

  /// Blocks (spinning with adaptive backoff) until the counter reaches zero.
  ///
  /// On `std`, the backoff yields the OS thread after a short spin phase.
  /// On pure `no_std`, it continues spinning indefinitely.
  pub fn wait(&self) {
    // Fast path: counter already zero.
    if self.inner.counter.load(Ordering::Acquire) == 0 {
      return;
    }

    let mut iter = 0u32;
    while self.inner.counter.load(Ordering::Acquire) != 0 {
      backoff_step(&mut iter);
    }
  }
}
