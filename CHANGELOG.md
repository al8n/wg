# Changelog

All notable changes to this crate are documented here. The format is loosely
based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this
crate adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.1]

### Fixed

- **`wait_blocking()` no longer breaks builds on `wasm32` targets.**
  `EventListener::wait()` (blocking) does not exist on `target_family =
  "wasm"` — the method is now gated with
  `#[cfg(all(feature = "std", not(target_family = "wasm")))]` so
  `cargo check --all-features --target wasm32-unknown-unknown` succeeds.
- **`alloc + future` is now a testable configuration.** `tests/future.rs`
  previously imported `std::time`, `std::sync::atomic`, and runtime
  crates unconditionally, so `cargo test --no-default-features --features
  alloc,future` failed to compile. Std-only tests are now gated behind
  `#[cfg(feature = "std")]`, and the manual-poll / remaining / clone /
  fmt / over-done / add-assign tests use `core::*` / `alloc::*` so they
  compile and run under alloc-only.
- **Counter overflow in `add()` is now caught in release builds.** All
  three WaitGroup variants (`sync`, `spin`, `future`) used
  `debug_assert!` or unchecked `+=` / `fetch_add` for the overflow
  check, which silently wrapped in release mode. A wrap from
  `usize::MAX + 1 → 0` could reset the counter and let `wait()` return
  prematurely or hang. `sync::WaitGroup` now uses
  `checked_add(...).expect(...)`, and `spin` / `future` use
  `fetch_update` with `checked_add` so the atomic is never corrupted.

### Changed

- **CI now tests feature pairs and wasm with `--all-features`.** Added
  explicit `cargo build/test --no-default-features --features
  alloc,future` and `cargo build/test --all-features` steps alongside
  the existing `cargo hack --each-feature` runs. The cross-build job
  now also runs `cargo check --all-features` per target, catching
  platform-specific conditional-compilation misses like the wasm
  `wait_blocking` issue above.

## [1.0.0]

First stable release. Significant breaking changes from 0.9.x — see **Migration
from 0.9** below before upgrading.

The crate now commits to [Semantic Versioning](https://semver.org/): breaking
changes to the public API or a bump of the declared MSRV will require a major
version bump.

### Breaking changes

- **`done()` now returns the remaining count, not the previous value.** In
  0.9.x, `future::WaitGroup::done()` / `no_std::WaitGroup::done()` returned the
  counter value **before** the decrement, which contradicted the documented
  behaviour. It now consistently returns the count *after* decrementing. The
  blocking `WaitGroup` was already correct.

- **`waitings()` renamed to `remaining()`** on all variants. The new name
  reads naturally (`if wg.remaining() > 0 { … }`) and is consistent across
  variants.

- **Old `no_std` module replaced by `spin`.** The lock-free, atomic-counter
  variant now lives at `wg::spin::WaitGroup`. In `no_std` builds,
  `wg::WaitGroup` is a re-export of `wg::spin::WaitGroup` (backward-compatible
  for `no_std` users).

- **`future::AsyncWaitGroup` renamed to `future::WaitGroup`.** Use the
  module-qualified name: `wg::future::WaitGroup`.

- The `alloc` feature no longer pulls in `crossbeam-utils`; the spin backoff is
  now inlined into the crate.

### Fixed

- **Busy-loop in `future::WaitGroup::poll`.** The previous `Pending` branch
  called `wake_by_ref()` before returning `Pending`, causing the executor to
  re-poll the future continuously and burn 100% CPU until `done()` reached
  zero. The waker is now only registered once (via the listener) and the
  future properly yields.
- **Wrong return value from `done()`** (see breaking changes above).
- **`std::sync::Mutex` poisoning no longer panics.** The blocking `WaitGroup`
  now recovers the guard on poisoning via `PoisonError::into_inner` — a
  poisoned counter is not a memory-safety concern, and cascading panics
  across every thread touching the group were an over-reaction.
- **Redundant `notify(usize::MAX)` on every `done()`.** The async variant now
  only notifies waiters when the counter actually reaches zero, instead of
  on every decrement.
- **`crossbeam_utils::Backoff::new()` was being reset inside the wait loop,**
  so it never escalated past its first spin budget. Replaced with an inline
  adaptive backoff that spins and — on `std` — yields the OS thread once the
  spin budget is exhausted.
- **Silent over-done no longer triggers the misleading `assert_eq!(x, 0)`.**
  Calling `done()` on a zero counter remains a silent no-op returning `0`.
- The `required-features = ["tokio"]` on `tests/future.rs` pointed at a
  non-existent feature, so the integration test never ran in CI. Fixed to
  `required-features = ["future"]`, and the CI job now exercises it.

### Changed

- Memory ordering tightened: `add` uses `Release`, `done` uses `AcqRel`/
  `Acquire`, `remaining` / `wait` loads use `Acquire`. The previous `SeqCst`
  everywhere was unnecessarily strong — `Release`/`Acquire` provides the
  required happens-before edges for this structure.
- `#![forbid(unsafe_code)]` is now enforced at the crate level.

### Added

- `wg::spin::WaitGroup` — a lock-free, atomic-counter WaitGroup available on
  both `std` and `no_std`. Uses an inline adaptive backoff that yields the OS
  thread on `std` and spins on pure `no_std`.
- Dedicated integration tests for each variant.
- Compile-time `Send + Sync` assertions for all three variants.
- Declared MSRV (`rust-version = "1.76.0"`, driven by `parking_lot` and
  `triomphe` floor requirements).

### Migration from 0.9

- Replace `wg::AsyncWaitGroup` with `wg::future::WaitGroup`.
- Replace `.waitings()` with `.remaining()`.
- If you relied on `done()` returning the pre-decrement value on the async /
  no_std variants, adjust your code — it now returns the post-decrement value,
  matching the blocking variant and the documented contract.
- `required-features = ["tokio"]` in consumer code should be
  `required-features = ["future"]`.

## [0.9.2] and earlier

See the [git history] for pre-1.0 releases.

[git history]: https://github.com/al8n/wg/commits/main
[1.0.1]: https://github.com/al8n/wg/releases/tag/v1.0.1
[1.0.0]: https://github.com/al8n/wg/releases/tag/v1.0.0
[0.9.2]: https://github.com/al8n/wg/releases/tag/v0.9.2
