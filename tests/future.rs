use agnostic_lite::{AsyncSpawner, RuntimeLite};
use wg::AsyncWaitGroup;

use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

async fn basic_in<S: RuntimeLite>() {
    let wg = AsyncWaitGroup::new();
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

#[async_std::test]
async fn async_std_basic() {
    basic_in::<agnostic_lite::async_std::AsyncStdRuntime>().await;
}

#[test]
fn sync_basic() {
    smol::block_on(basic_in::<agnostic_lite::smol::SmolRuntime>())
}

async fn reuse_in<S: RuntimeLite>() {
    let wg = AsyncWaitGroup::new();
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

#[async_std::test]
async fn async_std_reuse() {
    reuse_in::<agnostic_lite::async_std::AsyncStdRuntime>().await;
}

#[test]
fn sync_reuse() {
    smol::block_on(reuse_in::<agnostic_lite::smol::SmolRuntime>())
}

async fn nested_in<S: AsyncSpawner>() {
    let wg = AsyncWaitGroup::new();
    let ctr = Arc::new(AtomicUsize::new(0));
    for _ in 0..5 {
        let worker = wg.add(1);
        let ctrx = ctr.clone();
        S::spawn_detach(async move {
            let nested_worker = worker.add(1);
            let ctrxx = ctrx.clone();
            async_std::task::spawn(async move {
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

#[async_std::test]
async fn async_std_nested() {
    nested_in::<agnostic_lite::async_std::AsyncStdSpawner>().await;
}

#[test]
fn sync_nested() {
    smol::block_on(nested_in::<agnostic_lite::smol::SmolSpawner>())
}

async fn from_in<S: AsyncSpawner>() {
    let wg = AsyncWaitGroup::from(5);
    for _ in 0..5 {
        let t = wg.clone();
        S::spawn_detach(async move {
            t.done();
        });
    }
    wg.wait().await;
}

#[async_std::test]
async fn from_async_std() {
    from_in::<agnostic_lite::async_std::AsyncStdSpawner>().await;
}

#[tokio::test]
async fn from_tokio() {
    from_in::<agnostic_lite::tokio::TokioSpawner>().await;
}

#[test]
fn from_sync() {
    smol::block_on(from_in::<agnostic_lite::smol::SmolSpawner>())
}

#[test]
fn test_async_waitings() {
    let wg = AsyncWaitGroup::new();
    wg.add(1);
    wg.add(1);
    assert_eq!(wg.waitings(), 2);
}

async fn block_wait_in<S: AsyncSpawner>() {
    let wg = AsyncWaitGroup::new();
    let t_wg = wg.add(1);
    S::spawn_detach(async move {
        // do some time consuming task
        t_wg.done();
    });

    // wait other thread completes
    wg.wait_blocking();

    assert_eq!(wg.waitings(), 0);
}

#[async_std::test]
async fn block_wait_async_std() {
    block_wait_in::<agnostic_lite::async_std::AsyncStdSpawner>().await;
}

#[tokio::test]
async fn block_wait_tokio() {
    block_wait_in::<agnostic_lite::tokio::TokioSpawner>().await;
}

#[test]
fn block_wait_sync() {
    smol::block_on(block_wait_in::<agnostic_lite::smol::SmolSpawner>())
}

async fn wake_after_updating_in<S: agnostic_lite::AsyncSpawner>() {
    let wg = AsyncWaitGroup::new();
    for _ in 0..100000 {
        let worker = wg.add(1);
        S::spawn_detach(async move {
            async_std::task::sleep(std::time::Duration::from_millis(10)).await;
            let mut a = 0;
            for _ in 0..1000 {
                a += 1;
            }
            println!("{a}");
            async_std::task::sleep(std::time::Duration::from_millis(10)).await;
            worker.done();
        });
    }
    wg.wait().await;
}

#[async_std::test]
async fn wake_after_updating_async_std() {
    wake_after_updating_in::<agnostic_lite::async_std::AsyncStdSpawner>().await;
}

#[tokio::test]
async fn wake_after_updating_tokio() {
    wake_after_updating_in::<agnostic_lite::tokio::TokioSpawner>().await;
}

#[test]
fn wake_after_updating_sync() {
    smol::block_on(wake_after_updating_in::<agnostic_lite::smol::SmolSpawner>())
}

#[test]
fn test_clone_and_fmt() {
    let awg = AsyncWaitGroup::new();
    let awg1 = awg.clone();
    awg1.add(3);
    assert_eq!(format!("{:?}", awg), format!("{:?}", awg1));
}
