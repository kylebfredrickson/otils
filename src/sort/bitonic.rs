use crate::ObliviousOps;
use std::thread;

// TODO: figure out why parallel_bitonic_linear_pass is slower.
// TODO: check the number of spawned threads.

pub fn parallel_bitonic_sort<T: ObliviousOps + Send>(list: &mut [T], cond: bool, threads: usize) {
    if threads > 1 {
        if list.len() > 1 {
            let (l_half, r_half) = list.split_at_mut(list.len() / 2);

            let l_threads = threads / 2;
            let r_threads = threads - l_threads;

            thread::scope(|s| {
                s.spawn(|| parallel_bitonic_sort(l_half, cond, l_threads));
                parallel_bitonic_sort(r_half, !cond, r_threads)
            });
            parallel_bitonic_merge(l_half, r_half, cond, threads);
        }
    } else {
        bitonic_sort(list, cond);
    }
}

fn bitonic_sort<T: ObliviousOps>(list: &mut [T], cond: bool) {
    if list.len() > 1 {
        let (l_half, r_half) = list.split_at_mut(list.len() / 2);
        bitonic_sort(l_half, cond);
        bitonic_sort(r_half, !cond);
        bitonic_merge(l_half, r_half, cond);
    }
}

fn parallel_bitonic_merge<T: ObliviousOps + Send>(
    l_half: &mut [T],
    r_half: &mut [T],
    cond: bool,
    threads: usize,
) {
    if threads > 1 {
        if l_half.len() >= 1 && r_half.len() >= 1 {
            bitonic_pass(l_half, r_half, cond);
            // parallel_bitonic_pass(l_half, r_half, cond, threads);

            let l_threads = threads / 2;
            let r_threads = threads - l_threads;
            thread::scope(|s| {
                s.spawn(|| {
                    let (ll_quarter, lr_quarter) = l_half.split_at_mut(l_half.len() / 2);
                    parallel_bitonic_merge(ll_quarter, lr_quarter, cond, l_threads)
                });
                let (rl_quarter, rr_quarter) = r_half.split_at_mut(r_half.len() / 2);
                parallel_bitonic_merge(rl_quarter, rr_quarter, cond, r_threads)
            });
        }
    } else {
        bitonic_merge(l_half, r_half, cond);
    }
}

fn bitonic_merge<T: ObliviousOps>(l_half: &mut [T], r_half: &mut [T], cond: bool) {
    if l_half.len() >= 1 && r_half.len() >= 1 {
        bitonic_pass(l_half, r_half, cond);

        let (ll_quarter, lr_quarter) = l_half.split_at_mut(l_half.len() / 2);
        let (rl_quarter, rr_quarter) = r_half.split_at_mut(r_half.len() / 2);
        bitonic_merge(ll_quarter, lr_quarter, cond);
        bitonic_merge(rl_quarter, rr_quarter, cond);
    }
}

// // This makes it slower for some reason.
// fn parallel_bitonic_pass<T: ObliviousOps + Send>(
//     l_half: &mut [T],
//     r_half: &mut [T],
//     cond: bool,
//     threads: usize,
// ) {
//     // need to check that the chunks are big enough also
//     if threads > 1 {
//         let l_threads = threads / 2;
//         let r_threads = threads - l_threads;
//         let (ll_quarter, lr_quarter) = l_half.split_at_mut(l_half.len() / 2);
//         let (rl_quarter, rr_quarter) = r_half.split_at_mut(r_half.len() / 2);
//         thread::scope(|s| {
//             s.spawn(|| parallel_bitonic_pass(ll_quarter, rl_quarter, cond, l_threads));
//             parallel_bitonic_pass(lr_quarter, rr_quarter, cond, r_threads);
//         });
//     } else {
//         bitonic_pass(l_half, r_half, cond);
//     }
// }

#[inline]
fn bitonic_pass<T: ObliviousOps>(l_half: &mut [T], r_half: &mut [T], cond: bool) {
    for i in 0..l_half.len() {
        T::osort(cond, &mut l_half[i], &mut r_half[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_bitonic_sort(b: &mut Bencher) {
        let size = 0x100000;
        let mut v: Vec<i32> = (0..size).rev().collect();
        // let v = &mut v[..];

        b.iter(|| parallel_bitonic_sort(&mut v[..], true, 8));

        // let (l, r) = v.split_at_mut(v.len() / 2);
        // b.iter(|| parallel_bitonic_merge(l, r, 1, 8));
    }
}
