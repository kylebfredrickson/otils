pub trait ObliviousOps: Copy {
    fn oselect(cond: i8, a: Self, b: Self) -> Self;
    fn oequal(a: Self, b: Self) -> i8;
    fn ocompare(a: Self, b: Self) -> i8;

    fn ogreater(a: Self, b: Self) -> i8 {
        i8::oequal(Self::ocompare(a, b), 1)
    }

    fn olesser(a: Self, b: Self) -> i8 {
        i8::oequal(Self::ocompare(a, b), -1)
    }

    fn ogreater_equal(a: Self, b: Self) -> i8 {
        Self::ogreater(a, b) | Self::oequal(a, b)
    }

    fn olesser_equal(a: Self, b: Self) -> i8 {
        Self::olesser(a, b) | Self::oequal(a, b)
    }

    // This requires that Self implements the copy trait.
    fn oswap(cond: i8, a: &mut Self, b: &mut Self) {
        let tmp = *a;
        *a = Self::oselect(cond, *b, *a);
        *b = Self::oselect(cond, tmp, *b);
    }

    // When cond = 1, this is an ascending sort, when cond = -1 it is
    // descending.
    fn osort(cond: i8, a: &mut Self, b: &mut Self) {
        let cmp = Self::ocompare(*a, *b);
        Self::oswap(i8::oequal(cmp, cond), a, b);
    }

    fn omin(a: Self, b: Self) -> Self {
        let cmp = Self::ocompare(a, b);
        Self::oselect(i8::oequal(cmp, -1), a, b)
    }

    fn omax(a: Self, b: Self) -> Self {
        let cmp = Self::ocompare(a, b);
        Self::oselect(i8::oequal(cmp, 1), a, b)
    }
}

#[link(name = "ops", kind = "static")]
extern "C" {
    fn select_8(cond: i8, a: i8, b: i8) -> i8;
    fn select_16(cond: i8, a: i16, b: i16) -> i16;
    fn select_32(cond: i8, a: i32, b: i32) -> i32;
    fn select_64(cond: i8, a: i64, b: i64) -> i64;

    fn equal_8(a: i8, b: i8) -> i8;
    fn equal_16(a: i16, b: i16) -> i8;
    fn equal_32(a: i32, b: i32) -> i8;
    fn equal_64(a: i64, b: i64) -> i8;

    fn compare_8(a: i8, b: i8) -> i8;
    fn compare_16(a: i16, b: i16) -> i8;
    fn compare_32(a: i32, b: i32) -> i8;
    fn compare_64(a: i64, b: i64) -> i8;
}

// This implements ObliviousOps for primitive types by calling out to C
// functions. I decided to call to C because 1) Rust does not allow many of the
// operations that these bit manipulations require (overflow, casting) and 2) I
// was unsure if the Rust workarounds would actually be constant time
// (wrapping_sub, try_into, etc.).
macro_rules! impl_ops {
    ($from: ty, $into: ty, $select_fn: expr, $equal_fn: expr, $compare_fn: expr) => {
        impl ObliviousOps for $from {
            fn oselect(cond: i8, a: Self, b: Self) -> Self {
                unsafe { $select_fn(cond, a as $into, b as $into) as Self }
            }

            fn oequal(a: Self, b: Self) -> i8 {
                unsafe { $equal_fn(a as $into, b as $into) }
            }

            fn ocompare(a: Self, b: Self) -> i8 {
                unsafe { $compare_fn(a as $into, b as $into) }
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
                assert_eq!(<$t>::oselect(1, $a, $b), $a);
                assert_eq!(<$t>::oselect(0, $a, $b), $b);
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
    fn test_equal() {
        macro_rules! test_equal {
            ($t: ty, $a: expr, $b: expr) => {
                assert_eq!(<$t>::oequal($a, $a), 1);
                assert_eq!(<$t>::oequal($b, $b), 1);
                assert_eq!(<$t>::oequal($a, $b), 0);
                assert_eq!(<$t>::oequal($b, $a), 0);
            };
        }

        test_equal!(i8, -1, 2);
        test_equal!(i16, -1, 2);
        test_equal!(i32, -1, 2);
        test_equal!(i64, -1, 2);
        test_equal!(isize, -1, 2);

        test_equal!(u8, 1, 2);
        test_equal!(u16, 1, 2);
        test_equal!(u32, 1, 2);
        test_equal!(u64, 1, 2);
        test_equal!(usize, 1, 2);
    }

    #[test]
    fn test_compare() {
        macro_rules! test_compare {
            ($t: ty, $great: expr, $less: expr) => {
                assert_eq!(<$t>::ocompare($great, $less), 1);
                assert_eq!(<$t>::ocompare($less, $great), -1);
                assert_eq!(<$t>::ocompare($great, $great), 0);
                assert_eq!(<$t>::ocompare($less, $less), 0);
            };
        }

        test_compare!(i8, 4, 3);
        test_compare!(i16, 4, 3);
        test_compare!(i32, 4, 3);
        test_compare!(i64, 4, 3);
        test_compare!(isize, 4, 3);

        test_compare!(i8, -3, -4);
        test_compare!(i16, -3, -4);
        test_compare!(i32, -3, -4);
        test_compare!(i64, -3, -4);
        test_compare!(isize, -3, -4);

        test_compare!(u8, 4, 3);
        test_compare!(u16, 4, 3);
        test_compare!(u32, 4, 3);
        test_compare!(u64, 4, 3);
        test_compare!(usize, 4, 3);
    }

    #[test]
    fn test_greater() {
        macro_rules! test_greater {
            ($t: ty, $great: expr, $less: expr) => {
                assert_eq!(<$t>::ogreater($great, $less), 1);
                assert_eq!(<$t>::ogreater($less, $great), 0);
                assert_eq!(<$t>::ogreater($great, $great), 0);
                assert_eq!(<$t>::ogreater($less, $less), 0);
            };
        }

        test_greater!(i8, -2, -3);
        test_greater!(i16, -2, -3);
        test_greater!(i32, -2, -3);
        test_greater!(i64, -2, -3);
        test_greater!(isize, -2, -3);

        test_greater!(u8, 2, 1);
        test_greater!(u16, 2, 1);
        test_greater!(u32, 2, 1);
        test_greater!(u64, 2, 1);
        test_greater!(usize, 2, 1);
    }

    #[test]
    fn test_lesser() {
        macro_rules! test_lesser {
            ($t: ty, $great: expr, $less: expr) => {
                assert_eq!(<$t>::olesser($great, $less), 0);
                assert_eq!(<$t>::olesser($less, $great), 1);
                assert_eq!(<$t>::olesser($great, $great), 0);
                assert_eq!(<$t>::olesser($less, $less), 0);
            };
        }

        test_lesser!(i8, -2, -3);
        test_lesser!(i16, -2, -3);
        test_lesser!(i32, -2, -3);
        test_lesser!(i64, -2, -3);
        test_lesser!(isize, -2, -3);

        test_lesser!(u8, 2, 1);
        test_lesser!(u16, 2, 1);
        test_lesser!(u32, 2, 1);
        test_lesser!(u64, 2, 1);
        test_lesser!(usize, 2, 1);
    }

    #[test]
    fn test_greater_equal() {
        macro_rules! test_greater_equal {
            ($t: ty, $great: expr, $less: expr) => {
                assert_eq!(<$t>::ogreater_equal($great, $less), 1);
                assert_eq!(<$t>::ogreater_equal($less, $great), 0);
                assert_eq!(<$t>::ogreater_equal($great, $great), 1);
                assert_eq!(<$t>::ogreater_equal($less, $less), 1);
            };
        }

        test_greater_equal!(i8, -2, -3);
        test_greater_equal!(i16, -2, -3);
        test_greater_equal!(i32, -2, -3);
        test_greater_equal!(i64, -2, -3);
        test_greater_equal!(isize, -2, -3);

        test_greater_equal!(u8, 2, 1);
        test_greater_equal!(u16, 2, 1);
        test_greater_equal!(u32, 2, 1);
        test_greater_equal!(u64, 2, 1);
        test_greater_equal!(usize, 2, 1);
    }

    #[test]
    fn test_lesser_equal() {
        macro_rules! test_lesser_equal {
            ($t: ty, $great: expr, $less: expr) => {
                assert_eq!(<$t>::olesser_equal($great, $less), 0);
                assert_eq!(<$t>::olesser_equal($less, $great), 1);
                assert_eq!(<$t>::olesser_equal($great, $great), 1);
                assert_eq!(<$t>::olesser_equal($less, $less), 1);
            };
        }

        test_lesser_equal!(i8, -2, -3);
        test_lesser_equal!(i16, -2, -3);
        test_lesser_equal!(i32, -2, -3);
        test_lesser_equal!(i64, -2, -3);
        test_lesser_equal!(isize, -2, -3);

        test_lesser_equal!(u8, 2, 1);
        test_lesser_equal!(u16, 2, 1);
        test_lesser_equal!(u32, 2, 1);
        test_lesser_equal!(u64, 2, 1);
        test_lesser_equal!(usize, 2, 1);
    }

    #[test]
    fn test_swap() {
        macro_rules! test_swap {
            ($t: ty, $a: expr, $b: expr) => {
                let mut a = $a;
                let mut b = $b;
                <$t>::oswap(1, &mut a, &mut b);
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

                <$t>::osort(1, &mut less, &mut great);
                assert_eq!((less, great), ($less, $great));

                <$t>::osort(1, &mut great, &mut less);
                assert_eq!((great, less), ($less, $great));

                <$t>::osort(-1, &mut less, &mut great);
                assert_eq!((less, great), ($great, $less));

                <$t>::osort(-1, &mut great, &mut less);
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

    #[test]
    fn test_min() {
        macro_rules! test_min {
            ($t: ty, $min: expr, $max: expr) => {
                assert_eq!(<$t>::omin($min, $max), $min);
                assert_eq!(<$t>::omin($max, $min), $min);
            };
        }

        test_min!(i8, -1, 2);
        test_min!(i16, -1, 2);
        test_min!(i32, -1, 2);
        test_min!(i64, -1, 2);
        test_min!(isize, -1, 2);

        test_min!(u8, 1, 2);
        test_min!(u16, 1, 2);
        test_min!(u32, 1, 2);
        test_min!(u64, 1, 2);
        test_min!(usize, 1, 2);
    }

    #[test]
    fn test_max() {
        macro_rules! test_max {
            ($t: ty, $min: expr, $max: expr) => {
                assert_eq!(<$t>::omax($min, $max), $max);
                assert_eq!(<$t>::omax($max, $min), $max);
            };
        }

        test_max!(i8, -1, 2);
        test_max!(i16, -1, 2);
        test_max!(i32, -1, 2);
        test_max!(i64, -1, 2);
        test_max!(isize, -1, 2);

        test_max!(u8, 1, 2);
        test_max!(u16, 1, 2);
        test_max!(u32, 1, 2);
        test_max!(u64, 1, 2);
        test_max!(usize, 1, 2);
    }
}
