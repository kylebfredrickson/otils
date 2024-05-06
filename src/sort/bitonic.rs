use crate::ObliviousOps;
use std::{marker, thread};

// TODO: figure out why parallel_bitonic_linear_pass is slower.
// TODO: check the number of spawned threads.
// TODO: try an iterative implementation instead of a recursive implementation.

pub fn parallel_bitonic_sort<T: ObliviousOps + marker::Send>(
    list: &mut [T],
    cond: i8,
    threads: u8,
) {
    if threads > 1 {
        if list.len() > 1 {
            let l_threads = threads / 2;
            let r_threads = threads - l_threads;
            let (l_half, r_half) = list.split_at_mut(list.len() / 2);
            thread::scope(|s| {
                s.spawn(|| parallel_bitonic_sort(l_half, cond, l_threads));
                s.spawn(|| parallel_bitonic_sort(r_half, -cond, r_threads));
            });
            parallel_bitonic_merge(l_half, r_half, cond, threads);
        }
    } else {
        bitonic_sort(list, cond);
    }
}

fn bitonic_sort<T: ObliviousOps>(list: &mut [T], cond: i8) {
    if list.len() > 1 {
        let (l_half, r_half) = list.split_at_mut(list.len() / 2);
        bitonic_sort(l_half, cond);
        bitonic_sort(r_half, -cond);
        bitonic_merge(l_half, r_half, cond);
    }
}

fn parallel_bitonic_merge<T: ObliviousOps + marker::Send>(
    l_half: &mut [T],
    r_half: &mut [T],
    cond: i8,
    threads: u8,
) {
    if threads > 1 {
        if l_half.len() >= 1 && r_half.len() >= 1 {
            bitonic_linear_pass(l_half, r_half, cond);

            let l_threads = threads / 2;
            let r_threads = threads - l_threads;
            thread::scope(|s| {
                s.spawn(|| {
                    let (ll_quarter, lr_quarter) = l_half.split_at_mut(l_half.len() / 2);
                    parallel_bitonic_merge(ll_quarter, lr_quarter, cond, l_threads)
                });
                s.spawn(|| {
                    let (rl_quarter, rr_quarter) = r_half.split_at_mut(r_half.len() / 2);
                    parallel_bitonic_merge(rl_quarter, rr_quarter, cond, r_threads)
                });
            });
        }
    } else {
        bitonic_merge(l_half, r_half, cond);
    }
}

fn bitonic_merge<T: ObliviousOps>(l_half: &mut [T], r_half: &mut [T], cond: i8) {
    if l_half.len() >= 1 && r_half.len() >= 1 {
        bitonic_linear_pass(l_half, r_half, cond);

        let (ll_quarter, lr_quarter) = l_half.split_at_mut(l_half.len() / 2);
        let (rl_quarter, rr_quarter) = r_half.split_at_mut(r_half.len() / 2);
        bitonic_merge(ll_quarter, lr_quarter, cond);
        bitonic_merge(rl_quarter, rr_quarter, cond);
    }
}

// // This makes it slower for some reason.
// fn parallel_bitonic_linear_pass<T: ObliviousOps + marker::Send>(
//     l_half: &mut [T],
//     r_half: &mut [T],
//     cond: i8,
//     threads: u8,
// ) {
//     // need to check that the chunks are big enough also
//     if threads > 1 {
//         let l_threads = threads / 2;
//         let r_threads = threads - l_threads;
//         let (ll_quarter, lr_quarter) = l_half.split_at_mut(l_half.len() / 2);
//         let (rl_quarter, rr_quarter) = r_half.split_at_mut(r_half.len() / 2);
//         thread::scope(|s| {
//             s.spawn(|| parallel_bitonic_linear_pass(ll_quarter, rl_quarter, cond, l_threads));
//             s.spawn(|| parallel_bitonic_linear_pass(lr_quarter, rr_quarter, cond, r_threads));
//         });
//     } else {
//         bitonic_linear_pass(l_half, r_half, cond);
//     }
// }

#[inline]
fn bitonic_linear_pass<T: ObliviousOps>(l_half: &mut [T], r_half: &mut [T], cond: i8) {
    for i in 0..l_half.len() {
        T::osort(cond, &mut l_half[i], &mut r_half[i]);
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
