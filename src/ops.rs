pub trait ObliviousOps {
    fn select(a: Self, b: Self, choice: bool) -> Self;
}

macro_rules! impl_select {
    ($t: ty, $u: ty) => {
        impl ObliviousOps for $t {
            fn select(a: Self, b: Self, choice: bool) -> Self {
                let choice = choice as $t;
                (!(choice - 1) & a) | ((choice - 1) & b)
            }
        }

        impl ObliviousOps for $u {
            fn select(a: Self, b: Self, choice: bool) -> Self {
                let a = a as $t;
                let b = b as $t;
                <$t>::select(a, b, choice) as Self
            }
        }
    };
}

impl_select!(i8, u8);
impl_select!(i16, u16);
impl_select!(i32, u32);
impl_select!(i64, u64);
impl_select!(i128, u128);
impl_select!(isize, usize);

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_select_signed() {
        assert_eq!(i8::select(-2, -1, true), -2);
        assert_eq!(i8::select(-2, -1, false), -1);
        assert_eq!(i16::select(-2, -1, true), -2);
        assert_eq!(i16::select(-2, -1, false), -1);
        assert_eq!(i32::select(-2, -1, true), -2);
        assert_eq!(i32::select(-2, -1, false), -1);
        assert_eq!(i64::select(-2, -1, true), -2);
        assert_eq!(i64::select(-2, -1, false), -1);
        assert_eq!(i128::select(-2, -1, true), -2);
        assert_eq!(i128::select(-2, -1, false), -1);
        assert_eq!(isize::select(-2, -1, true), -2);
        assert_eq!(isize::select(-2, -1, false), -1);

        assert_eq!(u8::select(2, 1, true), 2);
        assert_eq!(u8::select(2, 1, false), 1);
        assert_eq!(u16::select(2, 1, true), 2);
        assert_eq!(u16::select(2, 1, true), 2);
        assert_eq!(u32::select(2, 1, true), 2);
        assert_eq!(u32::select(2, 1, true), 2);
        assert_eq!(u64::select(2, 1, false), 1);
        assert_eq!(u64::select(2, 1, false), 1);
        assert_eq!(u128::select(2, 1, false), 1);
        assert_eq!(u128::select(2, 1, false), 1);
        assert_eq!(usize::select(2, 1, false), 1);
        assert_eq!(usize::select(2, 1, false), 1);
    }
}
