#![cfg(feature = "std")]

use std::{
  sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
  },
  time::Duration,
};
use wg::WaitGroup;

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
fn test_remaining() {
  let wg = WaitGroup::new();
  wg.add(1);
  wg.add(1);
  assert_eq!(wg.remaining(), 2);
}

/// `done()` on a zero counter is a silent no-op and returns 0.
#[test]
fn test_over_done_is_silent() {
  let wg = WaitGroup::new();
  assert_eq!(wg.done(), 0);
  assert_eq!(wg.done(), 0);
  assert_eq!(wg.remaining(), 0);
}

/// `wait()` on a zero counter returns immediately without blocking.
#[test]
fn test_wait_on_zero_returns_immediately() {
  let wg = WaitGroup::new();
  let start = std::time::Instant::now();
  wg.wait();
  assert!(start.elapsed() < Duration::from_millis(50));
}

/// `+=` is shorthand for `add(n)`.
#[test]
fn test_add_assign() {
  let mut wg = WaitGroup::new();
  wg += 3;
  assert_eq!(wg.remaining(), 3);
  wg += 2;
  assert_eq!(wg.remaining(), 5);
}
