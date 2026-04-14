#![cfg(any(feature = "std", feature = "alloc"))]

//! Compile-time assertions that the public `WaitGroup` types are `Send + Sync`.
//!
//! These types are meant to be passed across threads and shared among them.
//! This file guards against accidental regressions — if someone later adds a
//! `!Send` or `!Sync` field (e.g. `Rc`, `RefCell`, a raw pointer), the build
//! will fail here instead of at user sites.

const fn _assert_send_sync<T: Send + Sync>() {}

#[test]
fn waitgroup_is_send_sync() {
    #[cfg(feature = "std")]
    _assert_send_sync::<wg::WaitGroup>();
    #[cfg(any(feature = "alloc", feature = "std"))]
    _assert_send_sync::<wg::spin::WaitGroup>();
    #[cfg(feature = "future")]
    {
        _assert_send_sync::<wg::future::WaitGroup>();
    }
}
