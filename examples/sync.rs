use wg::WaitGroup;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use std::thread::{spawn, sleep};

fn main() {
    let wg = WaitGroup::new();
    let ctr = Arc::new(AtomicUsize::new(0));

    for _ in 0..5 {
        let ctrx = ctr.clone();
        let t_wg = wg.add(1);
        spawn(move || {
            // mock some time consuming task
            sleep(Duration::from_millis(50));
            ctrx.fetch_add(1, Ordering::Relaxed);

            // mock task is finished
            t_wg.done();
        });
    }

    wg.wait();
    assert_eq!(ctr.load(Ordering::Relaxed), 5);
}