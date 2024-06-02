mod bitonic;
use crate::Max;
use bitonic::parallel_bitonic_sort;

pub fn sort<T: PartialOrd + Send + Max>(mut list: Vec<T>, threads: usize) -> Vec<T> {
    let list_len = list.len();
    let remaining = list_len.next_power_of_two() - list_len;
    list.extend((0..remaining).map(|_| T::maximum()));

    parallel_bitonic_sort(&mut list[..], true, threads);
    list.truncate(list_len);
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
        let a: Vec<i64> = (0..125).rev().collect();

        let a = sort(a, 2);
        assert!(is_sorted(&a));
    }
}
