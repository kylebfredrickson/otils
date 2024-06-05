use std::arch::asm;

#[link(name = "ops", kind = "static")]
extern "C" {
    fn swap8(cond: u8, a: *mut u8, b: *mut u8);
}

pub fn swap<T>(cond: bool, a: &mut T, b: &mut T) {
    assert!(std::mem::size_of::<T>() % 8 == 0);

    let mut remaining_blocks = std::mem::size_of::<T>() / 8;

    let mut a_ptr = a as *mut T as *mut i64;
    let mut b_ptr = b as *mut T as *mut i64;
    let cond = cond as u8;

    unsafe {
        while remaining_blocks > 0 {
            swap64(cond, a_ptr, b_ptr);
            a_ptr = a_ptr.add(1);
            b_ptr = b_ptr.add(1);
            remaining_blocks -= 1;
        }
    }
}

// slower
pub fn swap2<T>(cond: bool, a: &mut T, b: &mut T) {
    let mut remaining = std::mem::size_of::<T>();
    let mut a_ptr = a as *mut T as *mut u8;
    let mut b_ptr = b as *mut T as *mut u8;
    let cond = cond as u8;
    unsafe {
        while remaining > 0 {
            swap8(cond, a_ptr, b_ptr);
            a_ptr = a_ptr.add(1);
            b_ptr = b_ptr.add(1);
            remaining -= 1;
        }
    }
}

unsafe fn swap64(cond: u8, a: *mut i64, b: *mut i64) {
    asm!(
        "test {cond}, {cond}",
        "cmovnz {a:r}, {b:r}",
        "cmovnz {b:r}, {tmp:r}",
        cond = in(reg_byte) cond,
        tmp = in(reg) *a,
        a = inout(reg) *a,
        b = inout(reg) *b,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[test]
    fn test_swap() {
        macro_rules! test_swap {
            ($t: ty, $a: expr, $b: expr) => {
                let mut a = $a as $t;
                let mut b = $b as $t;

                swap(false, &mut a, &mut b);
                assert_eq!((a, b), ($a, $b));

                swap(true, &mut a, &mut b);
                assert_eq!((a, b), ($b, $a));
            };
        }

        test_swap!(i64, -5, 4);
        test_swap!(isize, -5, 4);
        test_swap!(u64, 5, 4);
        test_swap!(usize, 5, 4);
    }

    const SIZE: usize = 127;

    #[repr(align(64))]
    struct BigElem {
        _key: u64,
        _dum: [u64; SIZE],
    }

    #[bench]
    fn bench_swap(bench: &mut Bencher) {
        let mut a = BigElem {
            _key: 0,
            _dum: [0; SIZE],
        };

        let mut b = BigElem {
            _key: 1,
            _dum: [1; SIZE],
        };

        bench.iter(|| swap(true, &mut a, &mut b));
    }

    #[bench]
    fn bench_swap2(bench: &mut Bencher) {
        let mut a = BigElem {
            _key: 0,
            _dum: [0; SIZE],
        };

        let mut b = BigElem {
            _key: 1,
            _dum: [1; SIZE],
        };

        bench.iter(|| swap2(true, &mut a, &mut b));
    }
}
