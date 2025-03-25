mod swap;
use std::cmp::Ordering;

pub use swap::swap;
// pub use swap::ObliviousSwap;

pub trait ObliviousOps {
    fn oselect(cond: bool, a: Self, b: Self) -> Self;
}

#[link(name = "ops", kind = "static")]
unsafe extern "C" {
    unsafe fn select_8(cond: bool, a: i8, b: i8) -> i8;
    unsafe fn select_16(cond: bool, a: i16, b: i16) -> i16;
    unsafe fn select_32(cond: bool, a: i32, b: i32) -> i32;
    unsafe fn select_64(cond: bool, a: i64, b: i64) -> i64;
}

// This implements ObliviousOps for primitive types by calling out to C
// functions. I decided to call to C because 1) Rust does not allow many of the
// operations that these bit manipulations require (overflow, casting) and 2) I
// was unsure if the Rust workarounds would actually be constant time
// (wrapping_sub, try_into, etc.).
macro_rules! impl_ops {
    ($from: ty, $into: ty, $select_fn: expr) => {
        impl ObliviousOps for $from {
            fn oselect(cond: bool, a: Self, b: Self) -> Self {
                unsafe { $select_fn(cond, a as $into, b as $into) as Self }
            }
        }
    };
}

impl_ops!(i8, i8, select_8);
impl_ops!(u8, i8, select_8);
impl_ops!(i16, i16, select_16);
impl_ops!(u16, i16, select_16);
impl_ops!(i32, i32, select_32);
impl_ops!(u32, i32, select_32);
impl_ops!(i64, i64, select_64);
impl_ops!(u64, i64, select_64);
impl_ops!(isize, i64, select_64); // TODO this should be arch dependent.
impl_ops!(usize, i64, select_64); // TODO this should be arch dependent.

impl ObliviousOps for Ordering {
    fn oselect(cond: bool, a: Self, b: Self) -> Self {
        unsafe { select_8(cond, a as i8, b as i8).cmp(&0) }
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_select() {
        macro_rules! test_select {
            ($t: ty, $a: expr, $b: expr) => {
                assert_eq!(<$t>::oselect(true, $a, $b), $a);
                assert_eq!(<$t>::oselect(false, $a, $b), $b);
            };
        }

        test_select!(i8, -2, 1);
        test_select!(i16, -2, 1);
        test_select!(i32, -2, 1);
        test_select!(i64, -2, 1);
        test_select!(isize, -2, 1);

        test_select!(u8, 2, 1);
        test_select!(u16, 2, 1);
        test_select!(u32, 2, 1);
        test_select!(u64, 2, 1);
        test_select!(usize, 2, 1);

        test_select!(Ordering, Ordering::Equal, Ordering::Less);
        test_select!(Ordering, Ordering::Equal, Ordering::Greater);
        test_select!(Ordering, Ordering::Less, Ordering::Greater);
    }
}
