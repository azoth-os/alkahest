//! Useful synchronization primitives.
mod vlock;

pub use crate::sync::vlock::{BitMaskLock, VLock, Vault};
