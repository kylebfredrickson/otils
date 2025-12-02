use rand::rngs::OsRng;
use rand::TryRngCore;

use crate::compact;
use crate::ops;

pub fn or_shuffle<T>(data: &mut [T]) {
    let n = data.len();

    if n < 2 {
        return;
    }

    if n == 2 {
        let cond: bool = (OsRng.try_next_u32().unwrap() & 1) != 0;

        let (l_data, r_data) = data.split_at_mut(data.len() / 2);
        ops::swap(cond as bool, &mut l_data[0], &mut r_data[0]);
        return;
    }

    let bits = mark_half(n);
    compact::compact(data, &bits);

    let (l_data, r_data) = data.split_at_mut(data.len() / 2);
    or_shuffle(l_data);
    or_shuffle(r_data);
}

pub fn mark_half(n: usize) -> Vec<bool> {
    let mut m = vec![false; n];

    let mut ell = (n + 1) / 2;
    print!("{:?}: ", ell);

    for i in 0..n {
        let remaining = n - i;

        let r = OsRng.try_next_u64().unwrap() as usize % remaining; // SECURITY: Not unbiased.
        let take = r < ell;

        m[i] = take;
        ell -= take as usize;
    }
    println!("{:?}", m);

    m
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
