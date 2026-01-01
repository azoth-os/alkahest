use core::marker::Destruct;
use core::ops::{BitAnd, BitOr, Not};

pub const trait Address: [const] Destruct + Copy + PartialEq + Eq {
    type Value: Copy
        + [const] PartialEq
        + [const] Destruct
        + [const] BitAnd<Output = Self::Value>
        + [const] BitOr<Output = Self::Value>
        + [const] Not<Output = Self::Value>;

    fn as_value(&self) -> Self::Value;
    fn is_invalid(&self) -> bool;

    unsafe fn from_value_unchecked(value: Self::Value) -> Self;

    #[inline(always)]
    fn align_down(&self, align: Self::Value) -> Self {
        unsafe { Self::from_value_unchecked(self.as_value() & !align) }
    }
}
