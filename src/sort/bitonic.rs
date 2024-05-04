use crate::ObliviousOps;

// Implements bitonic sort.
pub fn bitonic_sort<T: ObliviousOps>(list: &mut [T], cond: i8) {
    if list.len() > 1 {
        let (first_half, second_half) = list.split_at_mut(list.len() / 2);
        bitonic_sort(first_half, cond);
        bitonic_sort(second_half, -cond);
        bitonic_merge(first_half, second_half, cond);
    }
}

fn bitonic_merge<T: ObliviousOps>(first_half: &mut [T], second_half: &mut [T], cond: i8) {
    if first_half.len() >= 1 && second_half.len() >= 1 {
        for i in 0..first_half.len() {
            T::osort(cond, &mut first_half[i], &mut second_half[i]);
        }
        let (first_quarter, second_quarter) = first_half.split_at_mut(first_half.len() / 2);
        let (third_quarter, fourth_quarter) = second_half.split_at_mut(second_half.len() / 2);
        bitonic_merge(first_quarter, second_quarter, cond);
        bitonic_merge(third_quarter, fourth_quarter, cond);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    fn is_sorted<T: Ord>(slice: &[T]) -> bool {
        slice.windows(2).all(|w| w[0] <= w[1])
    }

    #[bench]
    fn bench_bitonic_sort(b: &mut Bencher) {
        let size = 1048576;
        let mut v: Vec<i32> = (0..size).rev().collect();
        bitonic_sort(&mut v[..], 1);
        assert!(is_sorted(&v[..]));

        b.iter(|| bitonic_sort(&mut v[..], 0));
    }
}
