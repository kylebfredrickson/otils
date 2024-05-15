use crate::ObliviousOps;

mod or_compact;

use self::or_compact::parallel_or_compact;

pub fn ofilter<T: ObliviousOps + Send, F>(
    mut data: Vec<T>,
    f: F,
    num_matches: usize,
    threads: usize,
) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    let bits: Vec<usize> = data.iter().map(|x| f(x).try_into().unwrap()).collect();
    parallel_or_compact(&mut data[..], &bits[..], threads);
    data.truncate(num_matches);
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        macro_rules! test_filter {
            ($v: expr, $f: expr, $t: ty) => {
                let data: Vec<$t> = $v.into_iter().collect();
                let real: Vec<$t> = $v.into_iter().filter(|x| $f(x)).collect();
                let test = ofilter(data, $f, real.len(), 2);
                assert_eq!(test, real);
            };
        }

        test_filter!((1..101), |x| x % 2 == 0, i32);
        test_filter!((1..101), |x| x % 3 == 0, i32);
        test_filter!((1..101), |x| x % 7 == 0, i32);
    }
}
