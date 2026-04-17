#![cfg(all(any(feature = "std", feature = "alloc"), feature = "future"))]

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

use wg::future::WaitGroup;

// ==================================================================
// Tests that require std (runtime, timers, threads, Arc, AtomicUsize)
// ==================================================================

#[cfg(feature = "std")]
mod std_tests {
  use super::*;

  use agnostic_lite::{AsyncSpawner, RuntimeLite};

  use std::{
    sync::{
      atomic::{AtomicUsize, Ordering},
      Arc,
    },
    time::Duration,
  };

  async fn basic_in<S: RuntimeLite>() {
    let wg = WaitGroup::new();
    let ctr = Arc::new(AtomicUsize::new(0));

    for _ in 0..5 {
      let ctrx = ctr.clone();
      let wg = wg.add(1);

      S::spawn_detach(async move {
        S::sleep(Duration::from_millis(50)).await;
        ctrx.fetch_add(1, Ordering::Relaxed);
        wg.done();
      });
    }
    wg.wait().await;
    assert_eq!(ctr.load(Ordering::Relaxed), 5);
  }

  #[tokio::test]
  async fn tokio_basic() {
    basic_in::<agnostic_lite::tokio::TokioRuntime>().await;
  }

  #[test]
  fn smol_basic() {
    smol::block_on(basic_in::<agnostic_lite::smol::SmolRuntime>())
  }

  async fn reuse_in<S: RuntimeLite>() {
    let wg = WaitGroup::new();
    let ctr = Arc::new(AtomicUsize::new(0));
    for _ in 0..6 {
      let wg = wg.add(1);
      let ctrx = ctr.clone();
      S::spawn_detach(async move {
        S::sleep(Duration::from_millis(5)).await;
        ctrx.fetch_add(1, Ordering::Relaxed);
        wg.done();
      });
    }

    wg.wait().await;
    assert_eq!(ctr.load(Ordering::Relaxed), 6);

    let worker = wg.add(1);

    let ctrx = ctr.clone();
    S::spawn_detach(async move {
      S::sleep(Duration::from_millis(5)).await;
      ctrx.fetch_add(1, Ordering::Relaxed);
      worker.done();
    });

    wg.wait().await;
    assert_eq!(ctr.load(Ordering::Relaxed), 7);
  }

  #[tokio::test]
  async fn tokio_reuse() {
    reuse_in::<agnostic_lite::tokio::TokioRuntime>().await;
  }

  #[test]
  fn smol_reuse() {
    smol::block_on(reuse_in::<agnostic_lite::smol::SmolRuntime>())
  }

  async fn nested_in<S: AsyncSpawner>() {
    let wg = WaitGroup::new();
    let ctr = Arc::new(AtomicUsize::new(0));
    for _ in 0..5 {
      let worker = wg.add(1);
      let ctrx = ctr.clone();
      S::spawn_detach(async move {
        let nested_worker = worker.add(1);
        let ctrxx = ctrx.clone();
        S::spawn_detach(async move {
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
  async fn tokio_nested() {
    nested_in::<agnostic_lite::tokio::TokioSpawner>().await;
  }

  #[test]
  fn smol_nested() {
    smol::block_on(nested_in::<agnostic_lite::smol::SmolSpawner>())
  }

  async fn from_in<S: AsyncSpawner>() {
    let wg = WaitGroup::from(5);
    for _ in 0..5 {
      let t = wg.clone();
      S::spawn_detach(async move {
        t.done();
      });
    }
    wg.wait().await;
  }

  #[tokio::test]
  async fn from_tokio() {
    from_in::<agnostic_lite::tokio::TokioSpawner>().await;
  }

  #[test]
  fn from_smol() {
    smol::block_on(from_in::<agnostic_lite::smol::SmolSpawner>())
  }

  // `wait_blocking` is not available on wasm.
  #[cfg(not(target_family = "wasm"))]
  mod blocking_tests {
    use super::*;

    async fn block_wait_in<S: AsyncSpawner>() {
      let wg = WaitGroup::new();
      let t_wg = wg.add(1);
      S::spawn_detach(async move {
        t_wg.done();
        S::yield_now().await;
      });

      wg.wait_blocking();
      assert_eq!(wg.remaining(), 0);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn block_wait_tokio() {
      block_wait_in::<agnostic_lite::tokio::TokioSpawner>().await;
    }

    #[test]
    fn block_wait_smol() {
      smol::block_on(block_wait_in::<agnostic_lite::smol::SmolSpawner>())
    }

    #[test]
    fn test_wait_blocking_on_zero_returns_immediately() {
      let wg = WaitGroup::new();
      let start = std::time::Instant::now();
      wg.wait_blocking();
      assert!(start.elapsed() < Duration::from_millis(50));
    }
  }

  async fn wake_after_updating_in<S: RuntimeLite>() {
    let wg = WaitGroup::new();
    for _ in 0..100000 {
      let worker = wg.add(1);
      S::spawn_detach(async move {
        S::sleep(Duration::from_millis(10)).await;
        let mut a = 0;
        for _ in 0..1000 {
          a += 1;
        }
        debug_assert_eq!(a, 1000);
        S::sleep(Duration::from_millis(10)).await;
        worker.done();
      });
    }
    wg.wait().await;
  }

  #[tokio::test]
  async fn wake_after_updating_tokio() {
    wake_after_updating_in::<agnostic_lite::tokio::TokioRuntime>().await;
  }

  #[test]
  fn wake_after_updating_smol() {
    smol::block_on(wake_after_updating_in::<agnostic_lite::smol::SmolRuntime>())
  }
}

// ==================================================================
// Tests that work WITHOUT std — pure core/alloc, no runtime, no
// timers, no threads. These run under every feature combination that
// includes `alloc + future`.
// ==================================================================

#[test]
fn test_async_remaining() {
  let wg = WaitGroup::new();
  wg.add(1);
  wg.add(1);
  assert_eq!(wg.remaining(), 2);
}

#[test]
fn test_clone_and_fmt() {
  let awg = WaitGroup::new();
  let awg1 = awg.clone();
  awg1.add(3);
  assert_eq!(format!("{:?}", awg), format!("{:?}", awg1));
}

#[test]
fn test_over_done() {
  let wg = WaitGroup::new();
  assert_eq!(wg.done(), 0);
  assert_eq!(wg.done(), 0);
  assert_eq!(wg.remaining(), 0);
}

#[test]
fn test_add_assign() {
  let mut wg = WaitGroup::new();
  wg += 3;
  assert_eq!(wg.remaining(), 3);
}

// --------------------------------------------------------------------
// Manual-polling tests — exercise the race-dependent branches in
// `WaitGroupFuture::poll`. Uses core::task, not std::task, so they
// compile under alloc-only.
// --------------------------------------------------------------------

mod manual_poll {
  use super::*;
  use core::{
    future::Future,
    pin::Pin,
    ptr,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
  };

  extern crate alloc;
  use alloc::boxed::Box;

  const NOOP_VTABLE: RawWakerVTable = RawWakerVTable::new(
    |_| RawWaker::new(ptr::null(), &NOOP_VTABLE),
    |_| {},
    |_| {},
    |_| {},
  );

  fn noop_waker() -> Waker {
    unsafe { Waker::from_raw(RawWaker::new(ptr::null(), &NOOP_VTABLE)) }
  }

  #[test]
  fn poll_installs_fresh_listener_when_notified_but_counter_nonzero() {
    let wg = WaitGroup::new();
    wg.add(1);
    let mut fut = Box::pin(wg.wait());

    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);

    assert!(matches!(Pin::as_mut(&mut fut).poll(&mut cx), Poll::Pending));

    wg.done();
    wg.add(1);

    assert!(matches!(Pin::as_mut(&mut fut).poll(&mut cx), Poll::Pending));

    wg.done();
    assert!(matches!(
      Pin::as_mut(&mut fut).poll(&mut cx),
      Poll::Ready(())
    ));
  }
}
