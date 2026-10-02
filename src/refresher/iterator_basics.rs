pub fn first_even(nums: &[i32]) -> Option<i32> {
    todo!("Return the first even value")
}

pub fn all_positive(nums: &[i32]) -> bool {
    todo!("Return whether all values are positive")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_first_even_value() {
        assert_eq!(first_even(&[1, 3, 8, 10]), Some(8));
    }

    #[test]
    fn returns_none_when_no_even_value_exists() {
        assert_eq!(first_even(&[1, 3, 5]), None);
    }

    #[test]
    fn returns_none_for_empty_input() {
        assert_eq!(first_even(&[]), None);
    }

    #[test]
    fn all_values_are_positive() {
        assert!(all_positive(&[1, 2, 3, 4]));
    }

    #[test]
    fn zero_is_not_positive() {
        assert!(!all_positive(&[1, 0, 3]));
    }

    #[test]
    fn negative_value_is_not_positive() {
        assert!(!all_positive(&[1, -2, 3]));
    }
}
