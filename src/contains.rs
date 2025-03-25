// SECURITY: Check this doesn't short circuit.
pub fn contains<T: Eq>(list: &[T], item: &T) -> bool {
    list.iter().fold(false, |acc, elem| acc | (elem == item))
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
