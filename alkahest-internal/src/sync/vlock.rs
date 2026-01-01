//! VLock - Vault Locking using SFI techniques.
use crate::mem::addr::Address;
use core::marker::{Destruct, PhantomData};

pub const trait VLock<A: Address>: [const] Destruct {
    fn seal(&self, addr: A) -> A;
    fn unseal(&self, addr: A) -> Option<A>;
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
        if addr.is_invalid() {
            None
        } else {
            Some(Self {
                addr,
                lock,
                _mode: PhantomData,
            })
        }
    }

    ///
    #[inline(always)]
    pub fn secure(&self) -> A {
        self.lock.seal(self.addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct TestAddr(usize);

    impl const Address for TestAddr {
        type Value = usize;

        fn as_value(&self) -> Self::Value {
            self.0
        }

        unsafe fn from_value_unchecked(value: Self::Value) -> Self {
            TestAddr(value)
        }

        fn is_invalid(&self) -> bool {
            self.0 == 0
        }
    }

    struct OffsetLock {
        base: usize,
    }

    impl const VLock<TestAddr> for OffsetLock {
        fn seal(&self, addr: TestAddr) -> TestAddr {
            unsafe { TestAddr::from_value_unchecked(addr.as_value() + self.base) }
        }

        fn unseal(&self, addr: TestAddr) -> Option<TestAddr> {
            let val = addr.as_value();
            if val >= self.base {
                Some(unsafe { TestAddr::from_value_unchecked(val - self.base) })
            } else {
                None
            }
        }
    }

    #[test]
    fn test_vault_seal_unseal() {
        let base_addr = 0x1000;
        let lock = OffsetLock { base: base_addr };
        let raw_addr = TestAddr(0x42);

        let vault =
            Vault::<TestAddr, OffsetLock, ()>::new(raw_addr, lock).expect("Vault creation failed");

        // Vérification du scellage (secure)
        let sealed = vault.secure();
        assert_eq!(sealed.as_value(), 0x1042);

        // Vérification du déscellage
        let unsealed = vault.lock.unseal(sealed).unwrap();
        assert_eq!(unsealed, raw_addr);
    }

    #[test]
    fn test_vault_invalid_addr() {
        let lock = OffsetLock { base: 0x1000 };
        let invalid_addr = TestAddr(0); // is_invalid retournera true

        let vault = Vault::<TestAddr, OffsetLock, ()>::new(invalid_addr, lock);
        assert!(vault.is_none());
    }
}
