use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::{
    spawn,
    time::{sleep, Duration},
};
use wg::future::AsyncWaitGroup;

fn main() {
    async_std::task::block_on(async {
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
    });
}
