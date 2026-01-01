//! # Alkahest - Hypervisor Type-1
#![cfg_attr(not(test), no_std)]

/// a crucible is an execution context for the [Alkahest].
pub trait Crucible {}

#[allow(unused)]
pub struct Engine<C: Crucible> {
    context: C,
}

impl<C: Crucible> Engine<C> {
    pub fn new(context: C) -> Self {
        Self { context }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCrucible;
    impl Crucible for TestCrucible {}
    #[test]
    pub fn new_engine() {
        let _ = Engine::new(TestCrucible);
    }
}
