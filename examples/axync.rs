use wg::AsyncWaitGroup;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::{spawn, time::{sleep, Duration}};

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let wg = AsyncWaitGroup::new();
    let ctr = Arc::new(AtomicUsize::new(0));

    for _ in 0..5 {
        let ctrx = ctr.clone();
        let t_wg = wg.add(1);
        spawn(async move {
            // mock some time consuming task
            sleep(Duration::from_millis(50)).await;
            ctrx.fetch_add(1, Ordering::Relaxed);

            // mock task is finished
            t_wg.done();
        });
    }

    wg.wait().await;
    assert_eq!(ctr.load(Ordering::Relaxed), 5);
}