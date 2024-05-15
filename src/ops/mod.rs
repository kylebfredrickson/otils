pub trait ObliviousOps: Copy + PartialOrd {
    fn oselect(cond: bool, a: Self, b: Self) -> Self;

    // This requires that Self implements the copy trait.
    fn oswap(cond: bool, a: &mut Self, b: &mut Self) {
        let tmp = *a;
        if cond {
            *a = *b;
            *b = tmp;
        }
    }

    fn osort(cond: bool, a: &mut Self, b: &mut Self) {
        Self::oswap((a < b) ^ cond, a, b);
    }
}

#[link(name = "ops", kind = "static")]
extern "C" {
    fn select_8(cond: bool, a: i8, b: i8) -> i8;
    fn select_16(cond: bool, a: i16, b: i16) -> i16;
    fn select_32(cond: bool, a: i32, b: i32) -> i32;
    fn select_64(cond: bool, a: i64, b: i64) -> i64;
}

// This implements ObliviousOps for primitive types by calling out to C
// functions. I decided to call to C because 1) Rust does not allow many of the
// operations that these bit manipulations require (overflow, casting) and 2) I
// was unsure if the Rust workarounds would actually be constant time
// (wrapping_sub, try_into, etc.).
macro_rules! impl_ops {
    ($from: ty, $into: ty, $select_fn: expr, $equal_fn: expr, $compare_fn: expr) => {
        impl ObliviousOps for $from {
            fn oselect(cond: bool, a: Self, b: Self) -> Self {
                unsafe { $select_fn(cond, a as $into, b as $into) as Self }
            }
        }
    };
}

impl_ops!(i8, i8, select_8, equal_8, compare_8);
impl_ops!(u8, i8, select_8, equal_8, compare_8);
impl_ops!(i16, i16, select_16, equal_16, compare_16);
impl_ops!(u16, i16, select_16, equal_16, compare_16);
impl_ops!(i32, i32, select_32, equal_32, compare_32);
impl_ops!(u32, i32, select_32, equal_32, compare_32);
impl_ops!(i64, i64, select_64, equal_64, compare_64);
impl_ops!(u64, i64, select_64, equal_64, compare_64);
impl_ops!(isize, i64, select_64, equal_64, compare_64); // TODO this should be arch dependent.
impl_ops!(usize, i64, select_64, equal_64, compare_64); // TODO this should be arch dependent.

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
    }

    #[test]
    fn test_swap() {
        macro_rules! test_swap {
            ($t: ty, $a: expr, $b: expr) => {
                let mut a = $a;
                let mut b = $b;
                <$t>::oswap(true, &mut a, &mut b);
                assert_eq!((a, b), ($b, $a));
            };
        }

        test_swap!(i8, -5, 4);
        test_swap!(i16, -5, 4);
        test_swap!(i32, -5, 4);
        test_swap!(i64, -5, 4);
        test_swap!(isize, -5, 4);

        test_swap!(u8, 5, 4);
        test_swap!(u16, 5, 4);
        test_swap!(u32, 5, 4);
        test_swap!(u64, 5, 4);
        test_swap!(usize, 5, 4);
    }

    #[test]
    fn test_sort() {
        macro_rules! test_sort {
            ($t: ty, $less: expr, $great: expr) => {
                let mut less = $less;
                let mut great = $great;

                <$t>::osort(true, &mut less, &mut great);
                assert_eq!((less, great), ($less, $great));

                <$t>::osort(true, &mut great, &mut less);
                assert_eq!((great, less), ($less, $great));

                <$t>::osort(false, &mut less, &mut great);
                assert_eq!((less, great), ($great, $less));

                <$t>::osort(false, &mut great, &mut less);
                assert_eq!((great, less), ($great, $less));
            };
        }

        test_sort!(i8, -1, 2);
        test_sort!(i16, -1, 2);
        test_sort!(i32, -1, 2);
        test_sort!(i64, -1, 2);
        test_sort!(isize, -1, 2);

        test_sort!(u8, 1, 2);
        test_sort!(u16, 1, 2);
        test_sort!(u32, 1, 2);
        test_sort!(u64, 1, 2);
        test_sort!(usize, 1, 2);
    }
}
