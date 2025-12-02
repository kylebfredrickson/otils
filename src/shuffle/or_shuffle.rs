use rand::rngs::OsRng;
use rand::TryRngCore;
use rayon::ThreadPool;

use crate::compact;
use crate::ops;

pub fn mark_half(n: usize) -> Vec<bool> {
    let mut bits = vec![false; n];
    let mut ell = (n + 1) / 2;

    for i in 0..n {
        let remaining = n - i;

        let r = OsRng.try_next_u64().unwrap() as usize % remaining; // SECURITY: Not unbiased.
        let take = r < ell;

        bits[i] = take;
        ell -= take as usize;
    }
    println!("{:?}", bits);

    bits
}

pub fn parallel_or_shuffle<T: Send>(data: &mut [T], pool: &ThreadPool, threads: usize) {
    if threads <= 1 {
        or_shuffle(data);
        return;
    }

    let n = data.len();

    if n <= 2 {
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
    compact::compact(data, &bits);

    let (l_data, r_data) = data.split_at_mut(data.len() / 2);
    or_shuffle(l_data);
    or_shuffle(r_data);
}

// fn mark_half(n: usize) -> Vec<usize> {
//     let mut bits: Vec<usize> = vec![0; n];
//     let mut l = (n + 1) / 2;
//     print!("{:?}: ", l);
//     for (idx, b) in bits.iter_mut().enumerate() {
//         let r = (OsRng.try_next_u64().unwrap() & 1) as usize;
//         *b = ObliviousOps::oselect(r < l / (n - idx), 1, 0);
//         l = l - *b;
//     }
//     println!("{:?}", bits);
//     bits
// }
