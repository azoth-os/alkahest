use core::marker::Destruct;

pub const trait Address: [const] Destruct + Copy + PartialEq + Eq {
    fn as_usize(&self) -> usize;
    fn from_usize(addr: usize) -> Self;

    fn is_null(&self) -> bool {
        self.as_usize() == 0
    }
}
