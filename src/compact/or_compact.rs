use crate::ObliviousOps;
use std::{marker, thread};

// TODO: parallelize swapping.

pub fn parallel_or_compact<T: ObliviousOps + marker::Send>(
    data: &mut [T],
    bits: &[usize],
    threads: u8,
) {
    if threads > 1 {
        let n = data.len();
        if n > 0 {
            let n1: usize = 1 << usize::ilog2(data.len());
            let n2 = n - n1;
            let m: usize = bits[0..n2].iter().sum();

            let (first_data, second_data) = data.split_at_mut(n2);
            let (first_bits, second_bits) = bits.split_at(n2);
            or_compact(first_data, first_bits);
            parallel_or_off_compact(second_data, second_bits, (n1 - n2 + m) % n1, threads);
            for i in 0..n2 {
                T::oswap(
                    usize::ogreater_equal(i, m),
                    &mut first_data[i],
                    &mut second_data[n1 - n2 + i],
                );
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

        let (first_data, second_data) = data.split_at_mut(n2);
        let (first_bits, second_bits) = bits.split_at(n2);
        or_compact(first_data, first_bits);
        or_off_compact(second_data, second_bits, (n1 - n2 + m) % n1);
        for i in 0..n2 {
            T::oswap(
                usize::ogreater_equal(i, m),
                &mut first_data[i],
                &mut second_data[n1 - n2 + i],
            );
        }
    }
}

fn parallel_or_off_compact<T: ObliviousOps + marker::Send>(
    data: &mut [T],
    bits: &[usize],
    offset: usize,
    threads: u8,
) {
    if threads > 1 {
        let n = data.len();
        if n == 2 {
            let (first, second) = data.split_at_mut(1);
            let offset = (((1 - bits[0]) * bits[1]) ^ offset) as i8;
            T::oswap(offset, &mut first[0], &mut second[0]);
        } else if n > 2 {
            let m: usize = bits[0..(n / 2)].iter().sum();
            let (first_data, second_data) = data.split_at_mut(n / 2);
            let (first_bits, second_bits) = bits.split_at(n / 2);
            thread::scope(|s| {
                s.spawn(|| or_off_compact(first_data, first_bits, offset % (n / 2)));
                s.spawn(|| or_off_compact(second_data, second_bits, (offset + m) % (n / 2)));
            });

            let mut s = usize::ogreater_equal((offset % (n / 2)) + m, n / 2);
            s ^= usize::ogreater_equal(offset, n / 2);
            for i in 0..(n / 2) {
                let b = s ^ usize::ogreater_equal(i, (offset + m) % (n / 2));
                T::oswap(b, &mut first_data[i], &mut second_data[i]);
            }
        }
    } else {
        or_off_compact(data, bits, offset);
    }
}

fn or_off_compact<T: ObliviousOps>(data: &mut [T], bits: &[usize], offset: usize) {
    let n = data.len();
    if n == 2 {
        let (first, second) = data.split_at_mut(1);
        let b = (((1 - bits[0]) * bits[1]) ^ offset) as i8;
        T::oswap(b, &mut first[0], &mut second[0]);
    } else if n > 2 {
        let m: usize = bits[0..(n / 2)].iter().sum();
        let (first_data, second_data) = data.split_at_mut(n / 2);
        let (first_bits, second_bits) = bits.split_at(n / 2);
        or_off_compact(first_data, first_bits, offset % (n / 2));
        or_off_compact(second_data, second_bits, (offset + m) % (n / 2));

        let mut s = usize::ogreater_equal((offset % (n / 2)) + m, n / 2);
        s ^= usize::ogreater_equal(offset, n / 2);
        for i in 0..(n / 2) {
            let b = s ^ usize::ogreater_equal(i, (offset + m) % (n / 2));
            T::oswap(b, &mut first_data[i], &mut second_data[i]);
        }
    }
}

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

        b.iter(|| parallel_or_compact(&mut v[..], &bits[..], 4))
    }
}
