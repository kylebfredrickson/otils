use crate::ops;
use rayon::ThreadPool;

pub fn parallel_or_compact<T: Send>(
    data: &mut [T],
    bits: &[bool],
    pool: &ThreadPool,
    threads: usize,
) {
    let n = data.len();

    if threads <= 1 || n == 0 {
        or_compact(data, bits);
        return;
    }

    let n1 = 1 << usize::ilog2(data.len());
    let n2 = n - n1;
    let m = bits[0..n2].iter().map(|&b| b as usize).sum();

    let (l_data, r_data) = data.split_at_mut(n2);
    let (l_bits, r_bits) = bits.split_at(n2);
    pool.scope(|s| {
        or_compact(l_data, l_bits);
        s.spawn(|_| parallel_or_off_compact(r_data, r_bits, (n1 - n2 + m) % n1, pool, threads));
    });
    for i in 0..n2 {
        ops::swap(i >= m, &mut l_data[i], &mut r_data[n1 - n2 + i]);
    }
}

fn parallel_or_off_compact<T: Send>(
    data: &mut [T],
    bits: &[bool],
    offset: usize,
    pool: &ThreadPool,
    threads: usize,
) {
    let n = data.len();

    if threads <= 1 || n <= 2 {
        or_off_compact(data, bits, offset);
        return;
    }

    let m: usize = bits[0..(n / 2)].iter().map(|&b| b as usize).sum();
    let (l_data, r_data) = data.split_at_mut(n / 2);
    let (l_bits, r_bits) = bits.split_at(n / 2);
    let l_threads = threads / 2;
    let r_threads = threads - l_threads;

    pool.scope(|s| {
        s.spawn(|_| parallel_or_off_compact(l_data, l_bits, offset % (n / 2), pool, l_threads));
        s.spawn(|_| {
            parallel_or_off_compact(r_data, r_bits, (offset + m) % (n / 2), pool, r_threads)
        });
    });

    let mut s = (offset % (n / 2)) + m >= n / 2;
    s ^= offset >= n / 2;
    for i in 0..(n / 2) {
        let b = s ^ (i >= (offset + m) % (n / 2));
        ops::swap(b, &mut l_data[i], &mut r_data[i]);
    }
}

pub fn or_compact<T>(data: &mut [T], bits: &[bool]) {
    let n = data.len();

    if n == 0 {
        return;
    }

    let n1: usize = 1 << usize::ilog2(data.len());
    let n2 = n - n1;
    let m: usize = bits[0..n2].iter().map(|&b| b as usize).sum();

    let (l_data, r_data) = data.split_at_mut(n2);
    let (l_bits, r_bits) = bits.split_at(n2);
    or_compact(l_data, l_bits);
    or_off_compact(r_data, r_bits, (n1 - n2 + m) % n1);
    for i in 0..n2 {
        ops::swap(i >= m, &mut l_data[i], &mut r_data[n1 - n2 + i]);
    }
}

fn or_off_compact<T>(data: &mut [T], bits: &[bool], offset: usize) {
    let n = data.len();

    if n < 2 {
        return;
    } else if n == 2 {
        let (l_data, r_data) = data.split_at_mut(1);
        let b = (!bits[0] & bits[1]) ^ (offset != 0);
        ops::swap(b, &mut l_data[0], &mut r_data[0]);
        return;
    }

    let m: usize = bits[0..(n / 2)].iter().map(|&b| b as usize).sum();
    let (l_data, r_data) = data.split_at_mut(n / 2);
    let (l_bits, r_bits) = bits.split_at(n / 2);
    or_off_compact(l_data, l_bits, offset % (n / 2));
    or_off_compact(r_data, r_bits, (offset + m) % (n / 2));

    let mut s = (offset % (n / 2)) + m >= n / 2;
    s ^= offset >= n / 2;
    for i in 0..(n / 2) {
        let b = s ^ (i >= (offset + m) % (n / 2));
        ops::swap(b, &mut l_data[i], &mut r_data[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_or_compact(b: &mut Bencher) {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap();
        let size = 0x100000;
        let mut v: Vec<usize> = (0..size).collect();
        let bits: Vec<bool> = v.iter().map(|x| x % 2 != 0).collect();

        b.iter(|| parallel_or_compact(&mut v[..], &bits, &pool, 8))
    }

    struct BigElem {
        key: usize,
        _dum: [u64; 15],
    }

    impl BigElem {
        fn new(id: usize) -> Self {
            BigElem {
                key: id,
                _dum: [0; 15],
            }
        }
    }

    #[bench]
    fn bench_big_or_compact(b: &mut Bencher) {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap();
        let size = 0x100000;
        let mut v: Vec<BigElem> = (0..size).rev().map(|i| BigElem::new(i)).collect();
        let mut bits: Vec<bool> = v.iter().map(|x| x.key % 2 != 0).collect();

        b.iter(|| parallel_or_compact(&mut v[..], &mut bits, &pool, 8))
    }
}
