use crate::ObliviousOps;
use std::marker;

mod or_compact;

use self::or_compact::parallel_or_compact;

pub fn ofilter<T: ObliviousOps + marker::Send, F>(
    mut data: Vec<T>,
    f: F,
    num_matches: usize,
    threads: u8,
) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    let bits: Vec<usize> = data.iter().map(f).map(|b| if b { 1 } else { 0 }).collect();
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
                let real: Vec<$t> = $v.into_iter().filter($f).collect();
                let test = ofilter(data, $f, real.len(), 2);
                assert_eq!(test, real);
            };
        }

        test_filter!((1..11), |x| x % 2 == 0, i32);
        test_filter!((1..11), |x| x % 3 == 0, i32);
    }
}
