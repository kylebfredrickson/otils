mod bitonic;
mod padded_bitonic;
use bitonic::parallel_bitonic_sort;
use num::traits::Bounded;

pub fn sort<T: PartialOrd + Send + Bounded>(list: &mut [T], threads: usize) {
    assert!(list.len().next_power_of_two() == list.len());
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
