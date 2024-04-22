pub trait ObliviousOps {
    fn oselect(cond: bool, a: Self, b: Self) -> Self;
    fn oswap(cond: bool, a: &mut Self, b: &mut Self);
    fn oequal(a: Self, b: Self) -> bool;
    fn ocompare(a: Self, b: Self) -> i8;
    // fn olesser(a: Self, b: Self) // TODO
    // fn ogreater(a: Self, b: Self) // TODO
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

impl ObliviousOps for i8 {
    fn oselect(cond: bool, a: i8, b: i8) -> i8 {
        unsafe { select_8(cond, a, b) }
    }

    fn oswap(cond: bool, a: &mut i8, b: &mut i8) {
        let tmp = *a;
        *a = i8::oselect(cond, *b, *a);
        *b = i8::oselect(cond, tmp, *b);
    }

    fn oequal(a: i8, b: i8) -> bool {
        unsafe { equal_8(a, b) }
    }

    fn ocompare(a: i8, b: i8) -> i8 {
        unsafe { compare_8(a, b) }
    }
}

impl ObliviousOps for i16 {
    fn oselect(cond: bool, a: i16, b: i16) -> i16 {
        unsafe { select_16(cond, a, b) }
    }

    fn oswap(cond: bool, a: &mut i16, b: &mut i16) {
        let tmp = *a;
        *a = i16::oselect(cond, *b, *a);
        *b = i16::oselect(cond, tmp, *b);
    }

    fn oequal(a: i16, b: i16) -> bool {
        unsafe { equal_16(a, b) }
    }

    fn ocompare(a: i16, b: i16) -> i8 {
        unsafe { compare_16(a, b) }
    }
}

impl ObliviousOps for i32 {
    fn oselect(cond: bool, a: i32, b: i32) -> i32 {
        unsafe { select_32(cond, a, b) }
    }

    fn oswap(cond: bool, a: &mut i32, b: &mut i32) {
        let tmp = *a;
        *a = i32::oselect(cond, *b, *a);
        *b = i32::oselect(cond, tmp, *b);
    }

    fn oequal(a: i32, b: i32) -> bool {
        unsafe { equal_32(a, b) }
    }

    fn ocompare(a: i32, b: i32) -> i8 {
        unsafe { compare_32(a, b) }
    }
}

impl ObliviousOps for i64 {
    fn oselect(cond: bool, a: i64, b: i64) -> i64 {
        unsafe { select_64(cond, a, b) }
    }

    fn oswap(cond: bool, a: &mut i64, b: &mut i64) {
        let tmp = *a;
        *a = i64::oselect(cond, *b, *a);
        *b = i64::oselect(cond, tmp, *b);
    }

    fn oequal(a: i64, b: i64) -> bool {
        unsafe { equal_64(a, b) }
    }

    fn ocompare(a: i64, b: i64) -> i8 {
        unsafe { compare_64(a, b) }
    }
}

macro_rules! impl_unsigned {
    ($u: ty, $s: ty) => {
        impl ObliviousOps for $u {
            fn oselect(cond: bool, a: $u, b: $u) -> $u {
                let a = a as $s;
                let b = b as $s;
                <$s>::oselect(cond, a, b) as $u
            }

            fn oswap(cond: bool, a: &mut $u, b: &mut $u) {
                let tmp = *a;
                *a = <$u>::oselect(cond, *b, *a);
                *b = <$u>::oselect(cond, tmp, *b);
            }

            fn oequal(a: $u, b: $u) -> bool {
                let a = a as $s;
                let b = b as $s;
                <$s>::oequal(a, b)
            }

            fn ocompare(a: $u, b: $u) -> i8 {
                let a = a as $s;
                let b = b as $s;
                <$s>::ocompare(a, b)
            }
        }
    };
}

impl_unsigned!(u8, i8);
impl_unsigned!(u16, i16);
impl_unsigned!(u32, i32);
impl_unsigned!(u64, i64);

// TODO Add implementation of select for iterators over types that implement

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

        assert_eq!(u8::oselect(true, 2, 1), 2);
        assert_eq!(u8::oselect(false, 2, 1), 1);
        assert_eq!(u16::oselect(true, 2, 1), 2);
        assert_eq!(u16::oselect(false, 2, 1), 1);
        assert_eq!(u32::oselect(true, 2, 1), 2);
        assert_eq!(u32::oselect(false, 2, 1), 1);
        assert_eq!(u64::oselect(true, 2, 1), 2);
        assert_eq!(u64::oselect(false, 2, 1), 1);
    }

    #[test]
    fn test_eq() {
        assert!(i8::oequal(1, 1));
        assert!(!i8::oequal(1, 2));
        assert!(i16::oequal(1, 1));
        assert!(!i16::oequal(1, 2));
        assert!(i32::oequal(1, 1));
        assert!(!i32::oequal(1, 2));
        assert!(i64::oequal(1, 1));
        assert!(!i64::oequal(1, 2));

        assert!(u8::oequal(1, 1));
        assert!(!u8::oequal(1, 2));
        assert!(u16::oequal(1, 1));
        assert!(!u16::oequal(1, 2));
        assert!(u32::oequal(1, 1));
        assert!(!u32::oequal(1, 2));
        assert!(u64::oequal(1, 1));
        assert!(!u64::oequal(1, 2));
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
    }
}
