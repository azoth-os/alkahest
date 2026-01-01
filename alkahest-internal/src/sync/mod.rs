//! Useful synchronization primitives.

mod bmlock;
mod vlock;

pub use crate::sync::vlock::{VLock, Vault};
pub use crate::sync::bmlock::BMLock;