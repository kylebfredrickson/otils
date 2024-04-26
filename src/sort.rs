use crate::ops::ObliviousOps;

pub fn sort<T: ObliviousOps>(list: &mut [T]) {
    bitonic_sort(list, 1);
}

// Implements bitonic sort.
fn bitonic_sort<T: ObliviousOps>(list: &mut [T], up: i8) {
    if list.len() > 1 {
        let (first_half, second_half) = list.split_at_mut(list.len() / 2);
        bitonic_sort(first_half, up);
        bitonic_sort(second_half, -up);
        bitonic_merge(first_half, second_half, up);
    }
}

fn bitonic_merge<T: ObliviousOps>(first_half: &mut [T], second_half: &mut [T], up: i8) {
    if first_half.len() >= 1 && second_half.len() >= 1 {
        for i in 0..first_half.len() {
            T::osort(up, &mut first_half[i], &mut second_half[i]);
        }
        let (first_quarter, second_quarter) = first_half.split_at_mut(first_half.len() / 2);
        let (third_quarter, fourth_quarter) = second_half.split_at_mut(second_half.len() / 2);
        bitonic_merge(first_quarter, second_quarter, up);
        bitonic_merge(third_quarter, fourth_quarter, up);
    }
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
        sort(&mut a[..]);
        assert!(is_sorted(&a));
    }
}
