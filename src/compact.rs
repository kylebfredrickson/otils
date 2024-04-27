use crate::ObliviousOps;

mod or_compact;
use or_compact::or_compact;

pub fn ocompact<T: ObliviousOps>(data: &mut [T], bits: &mut [usize]) {
    or_compact(data, bits);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact() {
        let check_data = [1, 2, 3, 4, 5];
        let mut data = [63, 1, 42, 72, 2, 3, 67, 4, 5];
        let mut bits = [0, 1, 0, 0, 1, 1, 0, 1, 1];

        or_compact(&mut data, &mut bits);
        assert_eq!(check_data, data[..5]);

        let check_data = [2, 3, 4, 5];
        let mut data = [63, 1, 42, 72, 2, 3, 67, 4, 5];
        let mut bits = [0, 0, 0, 0, 1, 1, 0, 1, 1];

        or_compact(&mut data, &mut bits);
        assert_eq!(check_data, data[..4]);

        let check_data = [42, 2, 3, 4, 5];
        let mut data = [63, 1, 42, 72, 2, 3, 67, 4, 5];
        let mut bits = [0, 0, 1, 0, 1, 1, 0, 1, 1];

        or_compact(&mut data, &mut bits);
        assert_eq!(check_data, data[..5]);
    }
}
