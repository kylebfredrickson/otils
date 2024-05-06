use crate::ObliviousOps;

use std::marker;

// Implements bitonic sort
fn bitonic_sort<T: ObliviousOps>(list: &mut [T], cond: i8) {
    if list.len() > 1 {
        let (l_half, r_half) = list.split_at_mut(list.len() / 2);
        bitonic_sort(l_half, cond);
        bitonic_sort(r_half, -cond);
        bitonic_merge(l_half, r_half, cond);
    }
}

fn bitonic_merge<T: ObliviousOps>(l_half: &mut [T], r_half: &mut [T], cond: i8) {
    if l_half.len() >= 1 && r_half.len() >= 1 {
        for i in 0..l_half.len() {
            T::osort(cond, &mut l_half[i], &mut r_half[i]);
        }
        let (first_quarter, second_quarter) = l_half.split_at_mut(l_half.len() / 2);
        let (third_quarter, fourth_quarter) = r_half.split_at_mut(r_half.len() / 2);
        bitonic_merge(first_quarter, second_quarter, cond);
        bitonic_merge(third_quarter, fourth_quarter, cond);
    }
}

pub fn parallel_bitonic_sort<T: ObliviousOps + marker::Send>(
    list: &mut [T],
    cond: i8,
    threads: i8,
) {
    if threads > 1 {
        if list.len() > 1 {
            let l_threads = threads / 2;
            let r_threads = threads - l_threads;
            let (l_half, r_half) = list.split_at_mut(list.len() / 2);
            crossbeam::scope(|s| {
                s.spawn(|_| parallel_bitonic_sort(l_half, cond, l_threads));
                s.spawn(|_| parallel_bitonic_sort(r_half, -cond, r_threads));
            })
            .unwrap();
            parallel_bitonic_merge(l_half, r_half, cond, threads);
        }
    } else {
        bitonic_sort(list, cond);
    }
}

pub fn parallel_bitonic_merge<T: ObliviousOps + marker::Send>(
    l_half: &mut [T],
    r_half: &mut [T],
    cond: i8,
    threads: i8,
) {
    if threads > 1 {
        if l_half.len() >= 1 && r_half.len() >= 1 {
            for i in 0..l_half.len() {
                T::osort(cond, &mut l_half[i], &mut r_half[i]);
            }

            let l_threads = threads / 2;
            let r_threads = threads - l_threads;
            let (ll_quarter, lr_quarter) = l_half.split_at_mut(l_half.len() / 2);
            let (rl_quarter, rr_quarter) = r_half.split_at_mut(r_half.len() / 2);
            crossbeam::scope(|s| {
                s.spawn(|_| parallel_bitonic_merge(ll_quarter, lr_quarter, cond, l_threads));
                s.spawn(|_| parallel_bitonic_merge(rl_quarter, rr_quarter, cond, r_threads));
            })
            .unwrap();
        }
    } else {
        bitonic_merge(l_half, r_half, cond);
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
        parallel_bitonic_sort(&mut v[..], 1, 8);
        assert!(is_sorted(&v[..]));

        b.iter(|| parallel_bitonic_sort(&mut v[..], 1, 8));
    }
}
