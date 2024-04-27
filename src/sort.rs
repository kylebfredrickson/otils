use crate::ObliviousOps;

mod bitonic;
use bitonic::bitonic_sort;

pub fn osort<T: ObliviousOps>(list: &mut [T]) {
    bitonic_sort(list, 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted<T: Ord>(slice: &[T]) -> bool {
        slice.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_sort() {
        let mut a: [i64; 4] = [3, 1, 2, 4];
        osort(&mut a);
        assert!(is_sorted(&a));
    }
}
