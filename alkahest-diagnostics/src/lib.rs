//! # Alkahest-diagnostics
//!
//! This crate provides diagnostic utilities for the Alkahest hypervisor.
#![cfg_attr(not(test), no_std)]

mod diagnostic;

pub use crate::diagnostic::*;
