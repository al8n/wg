#![cfg(any(feature = "std", feature = "alloc"))]

use core::sync::atomic::{AtomicUsize, Ordering};
use wg::spin::WaitGroup;

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as std;

#[cfg(feature = "std")]
use std::{sync::Arc, time::Duration};

#[cfg(feature = "std")]
#[test]
fn basic() {
  let wg = WaitGroup::new();
  let ctr = Arc::new(AtomicUsize::new(0));

  for _ in 0..5 {
    let ctrx = ctr.clone();
    let t_wg = wg.add(1);
    std::thread::spawn(move || {
      std::thread::sleep(Duration::from_millis(5));
      ctrx.fetch_add(1, Ordering::Relaxed);
      t_wg.done();
    });
  }

  wg.wait();
  assert_eq!(ctr.load(Ordering::Relaxed), 5);
}

#[cfg(feature = "std")]
#[test]
fn reuse() {
  let wg = WaitGroup::new();
  let t_wg = wg.add(1);
  std::thread::spawn(move || {
    t_wg.done();
  });
  wg.wait();

  // New round on the same WaitGroup.
  let t_wg = wg.add(1);
  std::thread::spawn(move || {
    t_wg.done();
  });
  wg.wait();
  assert_eq!(wg.remaining(), 0);
}

#[cfg(feature = "std")]
#[test]
fn from_count() {
  let wg = WaitGroup::from(3);
  assert_eq!(wg.remaining(), 3);
  for _ in 0..3 {
    let t = wg.clone();
    std::thread::spawn(move || t.done());
  }
  wg.wait();
  assert_eq!(wg.remaining(), 0);
}

#[test]
fn done_returns_remaining() {
  let wg = WaitGroup::from(3);
  assert_eq!(wg.done(), 2);
  assert_eq!(wg.done(), 1);
  assert_eq!(wg.done(), 0);
  // Over-done is a silent no-op, returns 0.
  assert_eq!(wg.done(), 0);
}

#[test]
fn debug_and_clone() {
  let wg = WaitGroup::new();
  let clone = wg.clone();
  clone.add(2);
  // Both refer to the same counter.
  assert_eq!(format!("{:?}", wg), format!("{:?}", clone));
  assert_eq!(wg.remaining(), 2);
}

#[cfg(feature = "std")]
#[test]
fn wait_on_zero_returns_immediately() {
  let wg = WaitGroup::new();
  let start = std::time::Instant::now();
  wg.wait();
  assert!(start.elapsed() < Duration::from_millis(50));
}
