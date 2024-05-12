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
    F: Fn(&T) -> usize,
{
    let bits: Vec<usize> = data.iter().map(f).collect();
    parallel_or_compact(&mut data[..], &bits[..], threads);
    data.truncate(num_matches);
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        let data: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let odd_data = ofilter(data, |x| (x % 2).try_into().unwrap(), 5, 1);
        let check: Vec<i32> = (1..11).filter(|x| x % 2 == 1).collect();
        assert_eq!(odd_data, check);
    }
}
