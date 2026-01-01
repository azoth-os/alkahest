//! VLock - Vault Locking using SFI techniques.
use crate::mem::addr::Address;
use core::marker::{Destruct, PhantomData};

pub const trait VLock<A>: [const] Destruct {
    fn lock(&self, addr: A) -> A;
    fn unlock(&self, addr: A) -> Option<A>;
}

/// The vault who using a VLock for securing its address.
pub struct Vault<A, S, M>
where
    A: Address,
    S: VLock<A>,
{
    addr: A,
    lock: S,
    _mode: PhantomData<M>,
}

const impl<A, S, M> Vault<A, S, M>
where
    A: [const] Address,
    S: [const] VLock<A>,
{
    /// Creates a new vault with the given address and VLock strategy.
    #[inline(always)]
    pub fn new(addr: A, lock: S) -> Option<Self> {
        if addr.is_null() {
            None
        } else {
            Some(Self {
                addr,
                lock,
                _mode: PhantomData,
            })
        }
    }

    /// Returns the locked address.
    #[inline(always)]
    pub fn as_locked(&self) -> A {
        self.lock.lock(self.addr)
    }

    /// Attempts to unlock the locked address back to its original form.
    #[inline(always)]
    pub fn try_unlock(&self, addr: A) -> Option<A> {
        self.lock.unlock(addr)
    }

    pub fn secure(&self) -> A {
        self.lock.lock(self.addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct TestAddr(usize);

    impl const Address for TestAddr {
        fn as_usize(&self) -> usize {
            self.0
        }

        fn from_usize(addr: usize) -> Self {
            TestAddr(addr)
        }
    }

    struct OffsetLock;

    impl const VLock<TestAddr> for OffsetLock {
        fn lock(&self, addr: TestAddr) -> TestAddr {
            TestAddr(addr.as_usize() + 0x1000)
        }

        fn unlock(&self, addr: TestAddr) -> Option<TestAddr> {
            let val = addr.as_usize();
            if val >= 0x1000 {
                Some(TestAddr(val - 0x1000))
            } else {
                None
            }
        }
    }

    #[test]
    fn test_vault_creation_and_lock() {
        let addr = TestAddr(0x42);
        let lock = OffsetLock;

        let vault = Vault::<TestAddr, OffsetLock, ()>::new(addr, lock).expect("Should not be null");
        assert_eq!(vault.secure().as_usize(), 0x1042);
    }


    #[test]
    fn test_vault_null_protection() {
        let null_addr = TestAddr(0);
        let lock = OffsetLock;
        let vault = Vault::<TestAddr, OffsetLock, ()>::new(null_addr, lock);
        assert!(vault.is_none(), "Vault should refuse null addresses");
    }
}
