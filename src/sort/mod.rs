mod bitonic;
use bitonic::parallel_bitonic_sort;
use num::traits::Bounded;

pub fn sort<T: PartialOrd + Send + Bounded>(mut list: Vec<T>, threads: usize) -> Vec<T> {
    let remaining = list.len().next_power_of_two() - list.len();
    list.reserve(remaining);
    list.extend((0..remaining).map(|_| T::max_value()));
    parallel_bitonic_sort(&mut list[..], true, threads);
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted<T: Ord>(slice: &[T]) -> bool {
        slice.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_sort() {
        let a: Vec<i64> = (0..127).rev().collect();

        let a = sort(a, 2);
        assert!(is_sorted(&a));
    }
}
