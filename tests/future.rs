use wg::future::AsyncWaitGroup;

use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

#[async_std::test]
async fn test_async_wait_group() {
    let wg = AsyncWaitGroup::new();
    let ctr = Arc::new(AtomicUsize::new(0));

    for _ in 0..5 {
        let ctrx = ctr.clone();
        let wg = wg.add(1);

        async_std::task::spawn(async move {
            async_std::task::sleep(Duration::from_millis(50)).await;
            ctrx.fetch_add(1, Ordering::Relaxed);
            wg.done();
        });
    }
    wg.wait().await;
    assert_eq!(ctr.load(Ordering::Relaxed), 5);
}

#[async_std::test]
async fn test_async_wait_group_reuse() {
    let wg = AsyncWaitGroup::new();
    let ctr = Arc::new(AtomicUsize::new(0));
    for _ in 0..6 {
        let wg = wg.add(1);
        let ctrx = ctr.clone();
        async_std::task::spawn(async move {
            async_std::task::sleep(Duration::from_millis(5)).await;
            ctrx.fetch_add(1, Ordering::Relaxed);
            wg.done();
        });
    }

    wg.wait().await;
    assert_eq!(ctr.load(Ordering::Relaxed), 6);

    let worker = wg.add(1);

    let ctrx = ctr.clone();
    async_std::task::spawn(async move {
        async_std::task::sleep(Duration::from_millis(5)).await;
        ctrx.fetch_add(1, Ordering::Relaxed);
        worker.done();
    });

    wg.wait().await;
    assert_eq!(ctr.load(Ordering::Relaxed), 7);
}

#[async_std::test]
async fn test_async_wait_group_nested() {
    let wg = AsyncWaitGroup::new();
    let ctr = Arc::new(AtomicUsize::new(0));
    for _ in 0..5 {
        let worker = wg.add(1);
        let ctrx = ctr.clone();
        async_std::task::spawn(async move {
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

#[async_std::test]
async fn test_async_wait_group_from() {
    let wg = AsyncWaitGroup::from(5);
    for _ in 0..5 {
        let t = wg.clone();
        async_std::task::spawn(async move {
            t.done();
        });
    }
    wg.wait().await;
}

#[async_std::test]
async fn test_sync_wait_group() {
    let wg = AsyncWaitGroup::new();
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
    wg.wait().await;
    assert_eq!(ctr.load(Ordering::Relaxed), 5);
}

#[async_std::test]
async fn test_async_waitings() {
    let wg = AsyncWaitGroup::new();
    wg.add(1);
    wg.add(1);
    assert_eq!(wg.waitings(), 2);
}

#[test]
fn test_async_block_wait() {
    let wg = AsyncWaitGroup::new();
    let t_wg = wg.add(1);
    std::thread::spawn(move || {
        // do some time consuming task
        t_wg.done();
    });
    let spawner = |fut| {
        async_std::task::spawn(fut);
    };
    // wait other thread completes
    wg.block_wait(spawner);

    assert_eq!(wg.waitings(), 0);
}

#[async_std::test]
async fn test_wake_after_updating() {
    let wg = AsyncWaitGroup::new();
    for _ in 0..100000 {
        let worker = wg.add(1);
        async_std::task::spawn(async move {
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

#[test]
fn test_clone_and_fmt() {
    let awg = AsyncWaitGroup::new();
    let awg1 = awg.clone();
    awg1.add(3);
    assert_eq!(format!("{:?}", awg), format!("{:?}", awg1));
}
