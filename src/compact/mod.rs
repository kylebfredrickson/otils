mod or_compact;
use rayon::ThreadPool;

pub fn compact<T: Send>(data: &mut [T], bits: &[usize], pool: &ThreadPool, threads: usize) {
    // let bits: Vec<usize> = data.iter().map(|x| f(x).try_into().unwrap()).collect();
    or_compact::parallel_or_compact(&mut data[..], &bits[..], pool, threads);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();
        macro_rules! test_compact {
            ($v: expr, $f: expr, $t: ty) => {
                let mut data: Vec<$t> = $v.into_iter().collect();
                let real: Vec<$t> = $v.into_iter().filter(|x| $f(x)).collect();

                let bits: Vec<usize> = data.iter().map(|x| $f(x).try_into().unwrap()).collect();
                compact(&mut data[..], &bits[..], &pool, 2);
                assert_eq!(&data[0..real.len()], &real[..]);
            };
        }

        test_compact!((1..101), |x| x % 2 == 0, i64);
        test_compact!((1..101), |x| x % 3 == 0, i64);
        test_compact!((1..101), |x| x % 7 == 0, i64);
    }
}
