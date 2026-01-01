//! # Alkahest - Hypervisor Type-1
#![cfg_attr(not(test), no_std)]


pub struct AlkahestEngine<C: Crucible> {
    kernel: C
}

pub trait Crucible {}