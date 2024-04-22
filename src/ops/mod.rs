pub trait ObliviousOps {
    fn oselect(cond: bool, a: Self, b: Self) -> Self;
    fn oequal(a: Self, b: Self) -> bool;
    fn ocompare(a: Self, b: Self) -> i8;
    fn oswap(cond: bool, a: &mut Self, b: &mut Self);
    fn omin(a: Self, b: Self) -> Self;
    fn omax(a: Self, b: Self) -> Self;
}

#[link(name = "ops", kind = "static")]
extern "C" {
    fn select_8(cond: bool, a: i8, b: i8) -> i8;
    fn select_16(cond: bool, a: i16, b: i16) -> i16;
    fn select_32(cond: bool, a: i32, b: i32) -> i32;
    fn select_64(cond: bool, a: i64, b: i64) -> i64;

    fn equal_8(a: i8, b: i8) -> bool;
    fn equal_16(a: i16, b: i16) -> bool;
    fn equal_32(a: i32, b: i32) -> bool;
    fn equal_64(a: i64, b: i64) -> bool;

    fn compare_8(a: i8, b: i8) -> i8;
    fn compare_16(a: i16, b: i16) -> i8;
    fn compare_32(a: i32, b: i32) -> i8;
    fn compare_64(a: i64, b: i64) -> i8;
}

// This implicitly requires that Self implement the copy trait.
macro_rules! impl_oswap {
    () => {
        fn oswap(cond: bool, a: &mut Self, b: &mut Self) {
            let tmp = *a;
            *a = Self::oselect(cond, *b, *a);
            *b = Self::oselect(cond, tmp, *b);
        }
    };
}

macro_rules! impl_omin {
    () => {
        fn omin(a: Self, b: Self) -> Self {
            let cmp = Self::ocompare(a, b);
            Self::oselect(i8::oequal(cmp, -1), a, b)
        }
    };
}

macro_rules! impl_omax {
    () => {
        fn omax(a: Self, b: Self) -> Self {
            let cmp = Self::ocompare(a, b);
            Self::oselect(i8::oequal(cmp, 1), a, b)
        }
    };
}

// This implements ObliviousOps for primitive types by calling out to C
// functions. I decided to call to C because 1) Rust does not allow many of the
// operations that these bit manipulations require (e.g., overflow, casting,
// etc.) and 2) I was unsure if the Rust workarounds would actually be constant
// time (e.g., wrapping_sub, try_into, etc.).
macro_rules! impl_ops {
    ($from: ty, $into: ty, $select_fn: expr, $equal_fn: expr, $compare_fn: expr) => {
        impl ObliviousOps for $from {
            fn oselect(cond: bool, a: Self, b: Self) -> Self {
                unsafe { $select_fn(cond, a as $into, b as $into) as Self }
            }

            fn oequal(a: Self, b: Self) -> bool {
                unsafe { $equal_fn(a as $into, b as $into) }
            }

            fn ocompare(a: Self, b: Self) -> i8 {
                unsafe { $compare_fn(a as $into, b as $into) }
            }

            impl_oswap!();
            impl_omin!();
            impl_omax!();
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

// TODO Add implementation of select for iterators over types that implement
// impl<I> for ObliviousOps for I
// where
// I: Iterator,
// I::Item: ObliviousOps {
// }

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_select() {
        assert_eq!(i8::oselect(true, -2, -1), -2);
        assert_eq!(i8::oselect(false, -2, -1), -1);
        assert_eq!(i16::oselect(true, -2, -1), -2);
        assert_eq!(i16::oselect(false, -2, -1), -1);
        assert_eq!(i32::oselect(true, -2, -1), -2);
        assert_eq!(i32::oselect(false, -2, -1), -1);
        assert_eq!(i64::oselect(true, -2, -1), -2);
        assert_eq!(i64::oselect(false, -2, -1), -1);
        assert_eq!(isize::oselect(true, -2, -1), -2);
        assert_eq!(isize::oselect(false, -2, -1), -1);

        assert_eq!(u8::oselect(true, 2, 1), 2);
        assert_eq!(u8::oselect(false, 2, 1), 1);
        assert_eq!(u16::oselect(true, 2, 1), 2);
        assert_eq!(u16::oselect(false, 2, 1), 1);
        assert_eq!(u32::oselect(true, 2, 1), 2);
        assert_eq!(u32::oselect(false, 2, 1), 1);
        assert_eq!(u64::oselect(true, 2, 1), 2);
        assert_eq!(u64::oselect(false, 2, 1), 1);
        assert_eq!(usize::oselect(true, 2, 1), 2);
        assert_eq!(usize::oselect(false, 2, 1), 1);
    }

    #[test]
    fn test_equal() {
        assert!(i8::oequal(1, 1));
        assert!(!i8::oequal(1, 2));
        assert!(i16::oequal(1, 1));
        assert!(!i16::oequal(1, 2));
        assert!(i32::oequal(1, 1));
        assert!(!i32::oequal(1, 2));
        assert!(i64::oequal(1, 1));
        assert!(!i64::oequal(1, 2));
        assert!(isize::oequal(1, 1));
        assert!(!isize::oequal(1, 2));

        assert!(u8::oequal(1, 1));
        assert!(!u8::oequal(1, 2));
        assert!(u16::oequal(1, 1));
        assert!(!u16::oequal(1, 2));
        assert!(u32::oequal(1, 1));
        assert!(!u32::oequal(1, 2));
        assert!(u64::oequal(1, 1));
        assert!(!u64::oequal(1, 2));
        assert!(usize::oequal(1, 1));
        assert!(!usize::oequal(1, 2));
    }

    #[test]
    fn test_compare() {
        assert_eq!(i8::ocompare(4, 3), 1);
        assert_eq!(i8::ocompare(3, 3), 0);
        assert_eq!(i8::ocompare(3, 4), -1);
        assert_eq!(i16::ocompare(4, 3), 1);
        assert_eq!(i16::ocompare(3, 3), 0);
        assert_eq!(i16::ocompare(3, 4), -1);
        assert_eq!(i32::ocompare(4, 3), 1);
        assert_eq!(i32::ocompare(3, 3), 0);
        assert_eq!(i32::ocompare(3, 4), -1);
        assert_eq!(i64::ocompare(4, 3), 1);
        assert_eq!(i64::ocompare(3, 3), 0);
        assert_eq!(i64::ocompare(3, 4), -1);
        assert_eq!(isize::ocompare(4, 3), 1);
        assert_eq!(isize::ocompare(3, 3), 0);
        assert_eq!(isize::ocompare(3, 4), -1);

        assert_eq!(u8::ocompare(4, 3), 1);
        assert_eq!(u8::ocompare(3, 3), 0);
        assert_eq!(u8::ocompare(3, 4), -1);
        assert_eq!(u16::ocompare(4, 3), 1);
        assert_eq!(u16::ocompare(3, 3), 0);
        assert_eq!(u16::ocompare(3, 4), -1);
        assert_eq!(u32::ocompare(4, 3), 1);
        assert_eq!(u32::ocompare(3, 3), 0);
        assert_eq!(u32::ocompare(3, 4), -1);
        assert_eq!(u64::ocompare(4, 3), 1);
        assert_eq!(u64::ocompare(3, 3), 0);
        assert_eq!(u64::ocompare(3, 4), -1);
        assert_eq!(usize::ocompare(4, 3), 1);
        assert_eq!(usize::ocompare(3, 3), 0);
        assert_eq!(usize::ocompare(3, 4), -1);
    }

    #[test]
    fn test_swap() {
        let mut a = 5;
        let mut b = 4;
        i8::oswap(true, &mut a, &mut b);
        assert_eq!((a, b), (4, 5));

        let mut a = 5;
        let mut b = 4;
        i16::oswap(true, &mut a, &mut b);
        assert_eq!((a, b), (4, 5));

        let mut a = 5;
        let mut b = 4;
        i32::oswap(true, &mut a, &mut b);
        assert_eq!((a, b), (4, 5));

        let mut a = 5;
        let mut b = 4;
        i64::oswap(true, &mut a, &mut b);
        assert_eq!((a, b), (4, 5));

        let mut a = 5;
        let mut b = 4;
        isize::oswap(true, &mut a, &mut b);
        assert_eq!((a, b), (4, 5));

        let mut a = 5;
        let mut b = 4;
        u8::oswap(true, &mut a, &mut b);
        assert_eq!((a, b), (4, 5));

        let mut a = 5;
        let mut b = 4;
        u16::oswap(true, &mut a, &mut b);
        assert_eq!((a, b), (4, 5));

        let mut a = 5;
        let mut b = 4;
        u32::oswap(true, &mut a, &mut b);
        assert_eq!((a, b), (4, 5));

        let mut a = 5;
        let mut b = 4;
        u64::oswap(true, &mut a, &mut b);
        assert_eq!((a, b), (4, 5));

        let mut a = 5;
        let mut b = 4;
        usize::oswap(true, &mut a, &mut b);
        assert_eq!((a, b), (4, 5));
    }

    #[test]
    fn test_min() {
        assert_eq!(i8::omin(1, 2), 1);
        assert_eq!(i8::omin(2, 1), 1);
        assert_eq!(i16::omin(1, 2), 1);
        assert_eq!(i16::omin(2, 1), 1);
        assert_eq!(i32::omin(1, 2), 1);
        assert_eq!(i32::omin(2, 1), 1);
        assert_eq!(i64::omin(1, 2), 1);
        assert_eq!(i64::omin(2, 1), 1);
        assert_eq!(isize::omin(1, 2), 1);
        assert_eq!(isize::omin(2, 1), 1);

        assert_eq!(u8::omin(1, 2), 1);
        assert_eq!(u8::omin(2, 1), 1);
        assert_eq!(u16::omin(1, 2), 1);
        assert_eq!(u16::omin(2, 1), 1);
        assert_eq!(u32::omin(1, 2), 1);
        assert_eq!(u32::omin(2, 1), 1);
        assert_eq!(u64::omin(1, 2), 1);
        assert_eq!(u64::omin(2, 1), 1);
        assert_eq!(usize::omin(1, 2), 1);
        assert_eq!(usize::omin(2, 1), 1);
    }

    #[test]
    fn test_max() {
        assert_eq!(i8::omax(1, 2), 2);
        assert_eq!(i8::omax(2, 1), 2);
        assert_eq!(i16::omax(1, 2), 2);
        assert_eq!(i16::omax(2, 1), 2);
        assert_eq!(i32::omax(1, 2), 2);
        assert_eq!(i32::omax(2, 1), 2);
        assert_eq!(i64::omax(1, 2), 2);
        assert_eq!(i64::omax(2, 1), 2);
        assert_eq!(isize::omax(1, 2), 2);
        assert_eq!(isize::omax(2, 1), 2);

        assert_eq!(u8::omax(1, 2), 2);
        assert_eq!(u8::omax(2, 1), 2);
        assert_eq!(u16::omax(1, 2), 2);
        assert_eq!(u16::omax(2, 1), 2);
        assert_eq!(u32::omax(1, 2), 2);
        assert_eq!(u32::omax(2, 1), 2);
        assert_eq!(u64::omax(1, 2), 2);
        assert_eq!(u64::omax(2, 1), 2);
        assert_eq!(usize::omax(1, 2), 2);
        assert_eq!(usize::omax(2, 1), 2);
    }
}
