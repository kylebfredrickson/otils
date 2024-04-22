pub trait ObliviousOps {
    fn oselect(cond: bool, a: Self, b: Self) -> Self;
    fn oswap(cond: bool, a: &mut Self, b: &mut Self);
    fn oequal(a: Self, b: Self) -> bool;
    fn ocompare(a: Self, b: Self) -> i8;
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

macro_rules! impl_unsigned {
    ($u: ty, $s: ty) => {
        impl ObliviousOps for $u {
            fn oselect(cond: bool, a: Self, b: Self) -> Self {
                let a = a as $s;
                let b = b as $s;
                <$s>::oselect(cond, a, b) as Self
            }

            fn oequal(a: Self, b: Self) -> bool {
                let a = a as $s;
                let b = b as $s;
                <$s>::oequal(a, b)
            }

            fn ocompare(a: Self, b: Self) -> i8 {
                let a = a as $s;
                let b = b as $s;
                <$s>::ocompare(a, b)
            }

            impl_oswap!();
            impl_omin!();
            impl_omax!();
        }
    };
}

impl ObliviousOps for i8 {
    fn oselect(cond: bool, a: Self, b: Self) -> Self {
        unsafe { select_8(cond, a, b) }
    }

    fn oequal(a: Self, b: Self) -> bool {
        unsafe { equal_8(a, b) }
    }

    fn ocompare(a: Self, b: Self) -> i8 {
        unsafe { compare_8(a, b) }
    }

    impl_oswap!();
    impl_omin!();
    impl_omax!();
}

impl ObliviousOps for i16 {
    fn oselect(cond: bool, a: Self, b: Self) -> Self {
        unsafe { select_16(cond, a, b) }
    }

    fn oequal(a: Self, b: Self) -> bool {
        unsafe { equal_16(a, b) }
    }

    fn ocompare(a: Self, b: Self) -> i8 {
        unsafe { compare_16(a, b) }
    }

    impl_oswap!();
    impl_omin!();
    impl_omax!();
}

impl ObliviousOps for i32 {
    fn oselect(cond: bool, a: Self, b: Self) -> Self {
        unsafe { select_32(cond, a, b) }
    }

    fn oequal(a: Self, b: Self) -> bool {
        unsafe { equal_32(a, b) }
    }

    fn ocompare(a: Self, b: Self) -> i8 {
        unsafe { compare_32(a, b) }
    }

    impl_oswap!();
    impl_omin!();
    impl_omax!();
}

impl ObliviousOps for i64 {
    fn oselect(cond: bool, a: Self, b: Self) -> Self {
        unsafe { select_64(cond, a, b) }
    }

    fn oequal(a: Self, b: Self) -> bool {
        unsafe { equal_64(a, b) }
    }

    fn ocompare(a: Self, b: Self) -> i8 {
        unsafe { compare_64(a, b) }
    }

    impl_oswap!();
    impl_omin!();
    impl_omax!();
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

    #[test]
    fn test_omin() {
        assert_eq!(i8::omin(1, 2), 1);
        assert_eq!(i8::omin(2, 1), 1);
        assert_eq!(i16::omin(1, 2), 1);
        assert_eq!(i16::omin(2, 1), 1);
        assert_eq!(i32::omin(1, 2), 1);
        assert_eq!(i32::omin(2, 1), 1);
        assert_eq!(i64::omin(1, 2), 1);
        assert_eq!(i64::omin(2, 1), 1);

        assert_eq!(u8::omin(1, 2), 1);
        assert_eq!(u8::omin(2, 1), 1);
        assert_eq!(u16::omin(1, 2), 1);
        assert_eq!(u16::omin(2, 1), 1);
        assert_eq!(u32::omin(1, 2), 1);
        assert_eq!(u32::omin(2, 1), 1);
        assert_eq!(u64::omin(1, 2), 1);
        assert_eq!(u64::omin(2, 1), 1);
    }

    #[test]
    fn test_omax() {
        assert_eq!(i8::omax(1, 2), 2);
        assert_eq!(i8::omax(2, 1), 2);
        assert_eq!(i16::omax(1, 2), 2);
        assert_eq!(i16::omax(2, 1), 2);
        assert_eq!(i32::omax(1, 2), 2);
        assert_eq!(i32::omax(2, 1), 2);
        assert_eq!(i64::omax(1, 2), 2);
        assert_eq!(i64::omax(2, 1), 2);

        assert_eq!(u8::omax(1, 2), 2);
        assert_eq!(u8::omax(2, 1), 2);
        assert_eq!(u16::omax(1, 2), 2);
        assert_eq!(u16::omax(2, 1), 2);
        assert_eq!(u32::omax(1, 2), 2);
        assert_eq!(u32::omax(2, 1), 2);
        assert_eq!(u64::omax(1, 2), 2);
        assert_eq!(u64::omax(2, 1), 2);
    }
}
