// SECURITY: Check this doesn't short circuit.
pub fn contains<T: Eq>(list: &[T], item: &T) -> bool {
    list.iter()
        .fold(0u8, |acc, elem| acc | (elem == item) as u8)
        == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let list: Vec<i32> = (0..100).collect();
        assert!(contains(&list, &0));
        assert!(!contains(&list, &101));
    }
}
