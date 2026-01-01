//! VLock - Vault Locking using SFI techniques.
use crate::mem::addr::Address;
use core::marker::{Destruct, PhantomData};
use core::ops::Deref;

pub const trait VLock<A: Address>: [const] Destruct {
    fn seal(&self, addr: A) -> A;
    fn unseal(&self, addr: A) -> Option<A>;
}

#[derive(Clone, Copy)]
pub struct BitMaskLock<A: Address> {
    mask: A::Value,
    base: A::Value,
}

const impl<A: [const] Address> BitMaskLock<A> {
    pub fn new(mask: A::Value, base: A::Value) -> Self {
        Self {
            mask,
            base: base & !mask,
        }
    }
}

impl<A: [const] Address> const VLock<A> for BitMaskLock<A> {
    #[inline(always)]
    fn seal(&self, addr: A) -> A {
        let raw = addr.as_value();
        let confined = (raw & self.mask) | self.base;

        unsafe { A::from_value_unchecked(confined) }
    }

    #[inline(always)]
    fn unseal(&self, addr: A) -> Option<A> {
        let val = addr.as_value();
        if (val & !self.mask) == self.base {
            Some(unsafe { A::from_value_unchecked(val & self.mask) })
        } else {
            None
        }
    }
}

/// The vault who using a VLock for securing its address.
pub struct Vault<A, S, T, M>
where
    A: Address,
    S: VLock<A>,
{
    addr: A,
    lock: S,
    _data: PhantomData<T>,
    _mode: PhantomData<M>,
}

const impl<A, S, T, M> Vault<A, S, T, M>
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
                _data: PhantomData,
                _mode: PhantomData,
            })
        }
    }

    #[inline(always)]
    pub fn as_ref(&self) -> &T {
        match self.lock.unseal(self.addr) {
            Some(valid_addr) => unsafe {
                let raw_val = valid_addr.as_value();
                let addr_usize = core::ptr::read(&raw_val as *const _ as *const usize);

                &*(addr_usize as *const T)
            },
            None => panic!("Vault Integrity Violation"),
        }
    }

    ///
    #[inline(always)]
    pub fn secure(&self) -> A {
        self.lock.seal(self.addr)
    }
}

impl<A, S, T, M> Deref for Vault<A, S, T, M>
where
    A: Address,
    S: VLock<A>,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Mock de l'implémentation Address pour le test ---
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    struct MockAddr(usize);

    // Simulation du trait Address tel qu'attendu par ton crate
    impl Address for MockAddr {
        type Value = usize;
        fn as_value(&self) -> Self::Value { self.0 }
        fn is_invalid(&self) -> bool { self.0 == 0 }
        unsafe fn from_value_unchecked(val: Self::Value) -> Self { MockAddr(val) }
    }

    #[test]
    fn test_bitmask_confinement_logic() {
        // Masque de 256 octets (0xFF) et base à 0x1000
        let mask = 0x00FF;
        let base = 0x1000;
        let lock = BitMaskLock::<MockAddr>::new(mask, base);

        // Une adresse qui dépasse largement du masque
        let unsafe_addr = MockAddr(0x99FF); 
        let sealed = lock.seal(unsafe_addr);

        // L'adresse doit être confinée à 0x10FF
        assert_eq!(sealed.as_value(), 0x10FF);
    }

    #[test]
    fn test_vault_integrity_check() {
        let mask = 0x00FF;
        let base = 0x2000;
        let lock = BitMaskLock::<MockAddr>::new(mask, base);
        
        let secret_value: u32 = 42;
        let secret_ptr = &secret_value as *const u32 as usize;
        
        let vault: Vault<MockAddr, _, u32, ()> = Vault::new(MockAddr(secret_ptr), lock).unwrap();

        let secured_addr = vault.secure();
        assert!(lock.unseal(secured_addr).is_some());

        let corrupted_addr = MockAddr(0x3000 | (secret_ptr & mask)); 
        let corrupt_lock = BitMaskLock::<MockAddr>::new(mask, base); // Même lock
        
        assert!(corrupt_lock.unseal(corrupted_addr).is_none());
    }

    #[test]
    fn test_vault_deref_access() {
        let mask = usize::MAX;
        let base = 0;
        let lock = BitMaskLock::<MockAddr>::new(mask, base);

        let my_data: u64 = 0xDEADBEEF;
        let addr = MockAddr(&my_data as *const u64 as usize);
        
        let vault: Vault<MockAddr, _, u64, ()> = Vault::new(addr, lock).unwrap();

        assert_eq!(*vault, 0xDEADBEEF);
    }

    #[test]
    #[should_panic(expected = "Vault Integrity Violation")]
    fn test_vault_panic_on_corruption() {
        let mask = 0x000F;
        let base = 0xAAAA_0000;
        let lock = BitMaskLock::<MockAddr>::new(mask, base);

     
        let vault: Vault<MockAddr, _, u32, ()> = Vault::new(MockAddr(0x1234), lock).unwrap();

        let _ = *vault;
    }
}