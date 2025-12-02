use rand::rngs::OsRng;
use rand::TryRngCore;
use rayon::ThreadPool;

use crate::compact;
use crate::ops;

pub fn mark_half(n: usize) -> Vec<bool> {
    let mut remaining_ones = n / 2;

    (0..n)
        .map(|i| {
            let remaining = n - i;
            let r = OsRng.try_next_u64().unwrap() as usize % remaining; // SECURITY: Not unbiased.
            let bit = r < remaining_ones;
            remaining_ones -= bit as usize;
            bit
        })
        .collect()
}

pub fn parallel_or_shuffle<T: Send>(data: &mut [T], pool: &ThreadPool, threads: usize) {
    let n = data.len();

    if threads <= 1 || n <= 2 {
        or_shuffle(data);
        return;
    }

    let bits = mark_half(n);
    compact::par_compact(data, &bits, pool, threads);

    let (l_data, r_data) = data.split_at_mut(n / 2);
    let l_threads = threads / 2;
    let r_threads = threads - l_threads;

    pool.scope(|s| {
        s.spawn(|_| parallel_or_shuffle(l_data, pool, l_threads));
        s.spawn(|_| parallel_or_shuffle(r_data, pool, r_threads));
    });
}

pub fn or_shuffle<T>(data: &mut [T]) {
    let n = data.len();

    if n < 2 {
        return;
    } else if n == 2 {
        let cond = (OsRng.try_next_u32().unwrap() & 1) != 0;

        let (l_data, r_data) = data.split_at_mut(data.len() / 2);
        ops::swap(cond, &mut l_data[0], &mut r_data[0]);
        return;
    }

    let bits = mark_half(n);
    println!("{:?}", bits);
    compact::compact(data, &bits);

    let (l_data, r_data) = data.split_at_mut(data.len() / 2);
    or_shuffle(l_data);
    or_shuffle(r_data);
}
