mod bitonic;
use bitonic::parallel_bitonic_sort;

pub fn sort<T: PartialOrd + Send>(list: &mut [T], threads: usize) {
    parallel_bitonic_sort(list, true, threads);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted<T: Ord>(slice: &[T]) -> bool {
        slice.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_sort() {
        let mut a: Vec<i64> = (0..127).rev().collect();

        sort(&mut a[..], 2);
        assert!(is_sorted(&a));
    }
}
