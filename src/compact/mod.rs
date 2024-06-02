mod or_compact;

pub fn compact<T: Send>(data: &mut [T], f: fn(&T) -> bool, threads: usize) {
    let bits: Vec<usize> = data.iter().map(|x| f(x).try_into().unwrap()).collect();
    or_compact::parallel_or_compact(&mut data[..], &bits[..], threads);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact() {
        macro_rules! test_compact {
            ($v: expr, $f: expr, $t: ty) => {
                let mut data: Vec<$t> = $v.into_iter().collect();
                let real: Vec<$t> = $v.into_iter().filter(|x| $f(x)).collect();
                compact(&mut data[..], $f, 2);
                assert_eq!(&data[0..real.len()], &real[..]);
            };
        }

        test_compact!((1..101), |x| x % 2 == 0, i64);
        test_compact!((1..101), |x| x % 3 == 0, i64);
        test_compact!((1..101), |x| x % 7 == 0, i64);
    }
}
