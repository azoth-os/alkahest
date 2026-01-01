use crate::{mem::addr::Address, sync::VLock};

/// Bit Mask Lock [`BMLOCK`]
pub struct BMLock<A: Address> {
    mask: A::Value,
    base: A::Value,
}

const impl<A: [const] Address> BMLock<A> {
    pub fn new(mask: A::Value, base: A::Value) -> Self {
        Self { mask, base }
    }
}

impl<A: [const] Address> const VLock<A> for BMLock<A> {
    #[inline(always)]
    fn seal(&self, addr: A) -> A {
        let raw = addr.as_value();
        let confined = (raw & self.mask) | self.base;

        unsafe { A::from_value_unchecked(confined) }
    }

    fn unseal(&self, addr: A) -> Option<A> {
        let val = addr.as_value();
        if (val & !self.mask) == self.base {
            Some(unsafe { A::from_value_unchecked(val & self.mask) })
        } else {
            None
        }
    }
}
