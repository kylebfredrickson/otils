use crate::ObliviousOps;
use std::thread;

pub fn parallel_or_compact<T: ObliviousOps + Send>(data: &mut [T], bits: &[usize], threads: usize) {
    if threads > 1 {
        let n = data.len();
        if n > 0 {
            let n1: usize = 1 << usize::ilog2(data.len());
            let n2 = n - n1;
            let m: usize = bits[0..n2].iter().sum();

            let (l_data, r_data) = data.split_at_mut(n2);
            let (l_bits, r_bits) = bits.split_at(n2);
            or_compact(l_data, l_bits);
            parallel_or_off_compact(r_data, r_bits, (n1 - n2 + m) % n1, threads);
            for i in 0..n2 {
                T::oswap(i >= m, &mut l_data[i], &mut r_data[n1 - n2 + i]);
            }
        }
    } else {
        or_compact(data, bits);
    }
}

fn or_compact<T: ObliviousOps>(data: &mut [T], bits: &[usize]) {
    let n = data.len();
    if n > 0 {
        let n1: usize = 1 << usize::ilog2(data.len());
        let n2 = n - n1;
        let m: usize = bits[0..n2].iter().sum();

        let (l_data, r_data) = data.split_at_mut(n2);
        let (l_bits, r_bits) = bits.split_at(n2);
        or_compact(l_data, l_bits);
        or_off_compact(r_data, r_bits, (n1 - n2 + m) % n1);
        for i in 0..n2 {
            T::oswap(i >= m, &mut l_data[i], &mut r_data[n1 - n2 + i]);
        }
    }
}

fn parallel_or_off_compact<T: ObliviousOps + Send>(
    data: &mut [T],
    bits: &[usize],
    offset: usize,
    threads: usize,
) {
    if threads > 1 {
        let n = data.len();
        if n == 2 {
            let (l_data, r_data) = data.split_at_mut(1);
            let offset = (((1 - bits[0]) * bits[1]) ^ offset) as i8;
            T::oswap(offset == 1, &mut l_data[0], &mut r_data[0]);
        } else if n > 2 {
            let m: usize = bits[0..(n / 2)].iter().sum();
            let (l_data, r_data) = data.split_at_mut(n / 2);
            let (l_bits, r_bits) = bits.split_at(n / 2);

            let l_threads = threads / 2;
            let r_threads = threads - l_threads;
            thread::scope(|s| {
                s.spawn(|| parallel_or_off_compact(l_data, l_bits, offset % (n / 2), l_threads));
                parallel_or_off_compact(r_data, r_bits, (offset + m) % (n / 2), r_threads)
            });

            let mut s = (offset % (n / 2)) + m >= n / 2;
            s ^= offset >= n / 2;
            for i in 0..(n / 2) {
                let b = s ^ (i >= (offset + m) % (n / 2));
                T::oswap(b, &mut l_data[i], &mut r_data[i]);
            }
        }
    } else {
        or_off_compact(data, bits, offset);
    }
}

fn or_off_compact<T: ObliviousOps>(data: &mut [T], bits: &[usize], offset: usize) {
    let n = data.len();
    if n == 2 {
        let (l_data, r_data) = data.split_at_mut(1);
        let b = (((1 - bits[0]) * bits[1]) ^ offset) as i8;
        T::oswap(b == 1, &mut l_data[0], &mut r_data[0]);
    } else if n > 2 {
        let m: usize = bits[0..(n / 2)].iter().sum();
        let (l_data, r_data) = data.split_at_mut(n / 2);
        let (l_bits, r_bits) = bits.split_at(n / 2);
        or_off_compact(l_data, l_bits, offset % (n / 2));
        or_off_compact(r_data, r_bits, (offset + m) % (n / 2));

        let mut s = (offset % (n / 2)) + m >= n / 2;
        s ^= offset >= n / 2;
        for i in 0..(n / 2) {
            let b = s ^ (i >= (offset + m) % (n / 2));
            T::oswap(b, &mut l_data[i], &mut r_data[i]);
        }
    }
}

// fn or_off_swap<T: ObliviousOps>(l_data: &mut [T], r_data: &mut [T], )

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_or_compact(b: &mut Bencher) {
        let size = 0x100000;
        let mut v: Vec<i64> = (0..size).collect();
        let bits: Vec<usize> = v.iter().map(|x| (x % 2).try_into().unwrap()).collect();

        b.iter(|| parallel_or_compact(&mut v[..], &bits[..], 8))
    }
}
