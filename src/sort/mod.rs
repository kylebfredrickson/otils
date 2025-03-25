mod bitonic;
use crate::Max;
use bitonic::parallel_bitonic_sort;
use rayon::ThreadPool;

pub fn sort<T: Ord + Send + Max>(mut list: Vec<T>, pool: &ThreadPool, threads: usize) -> Vec<T> {
    let list_len = list.len();
    let remaining = list_len.next_power_of_two() - list_len;
    list.extend((0..remaining).map(|_| T::maximum()));

    parallel_bitonic_sort(&mut list[..], true, pool, threads);
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
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();
        let a: Vec<i64> = (0..125).rev().collect();

        let a = sort(a, &pool, 2);
        assert!(is_sorted(&a));
    }
}
