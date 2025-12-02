use crate::ops;
use rayon::ThreadPool;

// TODO: figure out why parallel_bitonic_linear_pass is slower.
// TODO: check the number of spawned threads.

pub fn parallel_bitonic_sort<T: Ord + Send>(
    data: &mut [T],
    cond: bool,
    pool: &ThreadPool,
    threads: usize,
) {
    if threads <= 1 || data.len() <= 1 {
        bitonic_sort(data, cond);
        return;
    }

    let (l_half, r_half) = data.split_at_mut(data.len() / 2);
    let l_threads = threads / 2;
    let r_threads = threads - l_threads;

    pool.scope(|s| {
        s.spawn(|_| parallel_bitonic_sort(l_half, cond, pool, l_threads));
        s.spawn(|_| parallel_bitonic_sort(r_half, !cond, pool, r_threads));
    });
    parallel_bitonic_merge(l_half, r_half, cond, pool, threads);
}

fn parallel_bitonic_merge<T: Ord + Send>(
    l_half: &mut [T],
    r_half: &mut [T],
    cond: bool,
    pool: &ThreadPool,
    threads: usize,
) {
    if threads <= 1 || l_half.len() < 1 {
        bitonic_merge(l_half, r_half, cond);
        return;
    }

    parallel_bitonic_pass(l_half, r_half, cond, pool, threads);
    let l_threads = threads / 2;
    let r_threads = threads - l_threads;

    pool.scope(|s| {
        s.spawn(|_| {
            let (ll_quarter, lr_quarter) = l_half.split_at_mut(l_half.len() / 2);
            parallel_bitonic_merge(ll_quarter, lr_quarter, cond, pool, l_threads)
        });
        s.spawn(|_| {
            let (rl_quarter, rr_quarter) = r_half.split_at_mut(r_half.len() / 2);
            parallel_bitonic_merge(rl_quarter, rr_quarter, cond, pool, r_threads)
        });
    });
}

fn parallel_bitonic_pass<T: Ord + Send>(
    l_half: &mut [T],
    r_half: &mut [T],
    cond: bool,
    pool: &ThreadPool,
    threads: usize,
) {
    if threads <= 1 {
        bitonic_pass(l_half, r_half, cond);
        return;
    }

    // (0..l_half.len()).into_par_iter().for_each(|i| {
    //     ops::swap(
    //         (l_half[i] < r_half[i]) ^ cond,
    //         &mut l_half[i],
    //         &mut r_half[i],
    //     );
    // });

    let l_threads = threads / 2;
    let r_threads = threads - l_threads;

    let (ll_quarter, lr_quarter) = l_half.split_at_mut(l_half.len() / 2);
    let (rl_quarter, rr_quarter) = r_half.split_at_mut(r_half.len() / 2);
    pool.scope(|s| {
        s.spawn(|_| parallel_bitonic_pass(ll_quarter, rl_quarter, cond, pool, l_threads));
        s.spawn(|_| parallel_bitonic_pass(lr_quarter, rr_quarter, cond, pool, r_threads));
    });
}

pub fn bitonic_sort<T: Ord>(list: &mut [T], cond: bool) {
    if list.len() <= 1 {
        return;
    }

    let (l_half, r_half) = list.split_at_mut(list.len() / 2);
    bitonic_sort(l_half, cond);
    bitonic_sort(r_half, !cond);
    bitonic_merge(l_half, r_half, cond);
}

fn bitonic_merge<T: Ord>(l_half: &mut [T], r_half: &mut [T], cond: bool) {
    if l_half.len() < 1 {
        return;
    }

    bitonic_pass(l_half, r_half, cond);

    let (ll_quarter, lr_quarter) = l_half.split_at_mut(l_half.len() / 2);
    let (rl_quarter, rr_quarter) = r_half.split_at_mut(r_half.len() / 2);
    bitonic_merge(ll_quarter, lr_quarter, cond);
    bitonic_merge(rl_quarter, rr_quarter, cond);
}

#[inline]
fn bitonic_pass<T: Ord>(l_half: &mut [T], r_half: &mut [T], cond: bool) {
    for i in 0..l_half.len() {
        ops::swap(
            (l_half[i] < r_half[i]) ^ cond,
            &mut l_half[i],
            &mut r_half[i],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use rayon::slice::ParallelSliceMut;
    use test::Bencher;

    #[bench]
    fn bench_bitonic_sort(b: &mut Bencher) {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap();

        let size = 0x100000;
        let mut v: Vec<i64> = (0..size).rev().collect();

        b.iter(|| parallel_bitonic_sort(&mut v[..], true, &pool, 8));
    }

    struct BigElem {
        key: u64,
        _dum: [u64; 15],
    }

    impl BigElem {
        fn new(id: u64) -> Self {
            BigElem {
                key: id,
                _dum: [0; 15],
            }
        }
    }

    impl Eq for BigElem {}

    impl PartialEq for BigElem {
        fn eq(&self, other: &Self) -> bool {
            self.key == other.key
        }
    }

    impl Ord for BigElem {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.key.cmp(&other.key)
        }
    }

    impl PartialOrd for BigElem {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    #[bench]
    fn bench_big_bitonic_sort(b: &mut Bencher) {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap();
        let size = 0x100000;
        let mut v: Vec<BigElem> = (0..size).rev().map(|i| BigElem::new(i)).collect();

        b.iter(|| parallel_bitonic_sort(&mut v[..], true, &pool, 8));
    }

    #[bench]
    fn bench_big_sort(b: &mut Bencher) {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap();
        let size = 0x100000;
        let mut v: Vec<BigElem> = (0..size).rev().map(|i| BigElem::new(i)).collect();

        b.iter(|| {
            pool.install(|| {
                v.par_sort();
            });
        });
    }
}
