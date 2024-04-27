use crate::ops::ObliviousOps;

pub fn ocompact(data: &mut [u64], bits: &mut [usize]) {
    or_compact(data, bits);
}

fn or_compact(data: &mut [u64], bits: &mut [usize]) {
    let n = data.len();
    if n > 0 {
        let n1: usize = 1 << usize::ilog2(data.len());
        let n2 = n - n1;
        let m: usize = bits[0..n2].iter().sum();

        let (first_data, second_data) = data.split_at_mut(n2);
        let (first_bits, second_bits) = bits.split_at_mut(n2);
        or_compact(first_data, first_bits);
        or_off_compact(second_data, second_bits, n1 - n2 + m);
        for i in 0..n2 {
            u64::oswap(
                usize::ogreater_equal(i, m),
                &mut first_data[i],
                &mut second_data[n1 - n2 + i],
            );
        }
    }
}

fn or_off_compact<T: ObliviousOps>(data: &mut [T], bits: &mut [usize], offset: usize) {
    let n = data.len();
    if n == 2 {
        let (first, second) = data.split_at_mut(1);
        let offset = (((1 - bits[0]) * bits[1]) ^ offset) as i8;
        T::oswap(offset, &mut first[0], &mut second[0]);
    } else if n > 2 {
        let m: usize = bits[0..(n / 2)].iter().sum();
        let (first_data, second_data) = data.split_at_mut(n / 2);
        let (first_bits, second_bits) = bits.split_at_mut(n / 2);
        or_off_compact(first_data, first_bits, offset % (n / 2));
        or_off_compact(second_data, second_bits, (offset + m) % (n / 2));

        let s = usize::ogreater_equal((offset % (n / 2)) + m, n / 2);
        let s = usize::ogreater_equal(offset, n / 2) ^ s;
        for i in 0..(n / 2) {
            let b = s ^ usize::ogreater_equal(i, (offset + m) % (n / 2));
            T::oswap(b, &mut first_data[i], &mut second_data[i]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact() {
        let check_data = [1, 2, 3, 4, 5];
        let mut data = [63, 1, 42, 72, 2, 3, 67, 4, 5];
        let mut bits = [0, 1, 0, 0, 1, 1, 0, 1, 1];

        or_compact(&mut data, &mut bits);
        assert_eq!(check_data, data[..5]);

        let check_data = [2, 3, 4, 5];
        let mut data = [63, 1, 42, 72, 2, 3, 67, 4, 5];
        let mut bits = [0, 0, 0, 0, 1, 1, 0, 1, 1];

        or_compact(&mut data, &mut bits);
        assert_eq!(check_data, data[..4]);

        let check_data = [42, 2, 3, 4, 5];
        let mut data = [63, 1, 42, 72, 2, 3, 67, 4, 5];
        let mut bits = [0, 0, 1, 0, 1, 1, 0, 1, 1];

        or_compact(&mut data, &mut bits);
        assert_eq!(check_data, data[..5]);
    }
}
