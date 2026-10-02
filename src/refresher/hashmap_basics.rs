use std::collections::HashMap;

pub fn count_occurrences(nums: &[i32]) -> HashMap<i32, usize> {
    todo!("Count how many times each value occurs")
}

pub fn most_frequent(nums: &[i32]) -> Option<i32> {
    todo!("Return a most frequent value")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_occurrences() {
        let result = count_occurrences(&[1, 2, 1, 3, 2, 1]);

        assert_eq!(result.get(&1), Some(&3));
        assert_eq!(result.get(&2), Some(&2));
        assert_eq!(result.get(&3), Some(&1));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn counts_empty_input() {
        assert!(count_occurrences(&[]).is_empty());
    }

    #[test]
    fn finds_most_frequent_value() {
        assert_eq!(most_frequent(&[4, 2, 4, 3, 4, 2]), Some(4));
    }

    #[test]
    fn returns_none_for_empty_input() {
        assert_eq!(most_frequent(&[]), None);
    }
}
