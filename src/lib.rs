/*
 * Copyright 2021 Al Liu (https://github.com/al8n). Licensed under MIT OR Apache-2.0.
 *
 *
 *
 * Copyright 2021 AwaitGroup authors (https://github.com/ibraheemdev/awaitgroup). Licensed under MIT.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#![doc = include_str!("../README.md")]
#![cfg_attr(not(all(feature = "std", test)), no_std)]
#![deny(missing_docs, warnings)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

/// A WaitGroup that can be used in async contexts. See [`future::WaitGroup`] for details.
#[cfg(all(any(feature = "std", feature = "alloc"), feature = "future"))]
#[cfg_attr(
  docsrs,
  doc(cfg(all(any(feature = "std", feature = "alloc"), feature = "future")))
)]
pub mod future;

#[cfg(feature = "std")]
mod sync;
#[cfg(feature = "std")]
pub use sync::*;

/// A lock-free, atomic-counter WaitGroup that spins on `wait`.
///
/// Available in both `std` and `no_std` environments. See
/// [`spin::WaitGroup`] for details.
#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
pub mod spin;

/// In `no_std` builds, `WaitGroup` is an alias for [`spin::WaitGroup`].
/// In `std` builds, `WaitGroup` is the `Mutex`/`Condvar`-based variant.
#[cfg(all(not(feature = "std"), feature = "alloc"))]
pub use spin::WaitGroup;
