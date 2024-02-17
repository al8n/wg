use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
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
fn test_waitings() {
    let wg = WaitGroup::new();
    wg.add(1);
    wg.add(1);
    assert_eq!(wg.waitings(), 2);
}
