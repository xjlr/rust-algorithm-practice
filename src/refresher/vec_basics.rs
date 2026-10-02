pub fn double_values(nums: Vec<i32>) -> Vec<i32> {
    todo!("Double every value")
}

pub fn sum_positive(nums: &[i32]) -> i32 {
    todo!("Return the sum of positive values")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_values() {
        assert_eq!(double_values(vec![1, 2, 3]), vec![2, 4, 6]);
    }

    #[test]
    fn doubles_negative_and_zero_values() {
        assert_eq!(double_values(vec![-2, 0, 5]), vec![-4, 0, 10]);
    }

    #[test]
    fn doubles_empty_vector() {
        assert_eq!(double_values(vec![]), Vec::<i32>::new());
    }

    #[test]
    fn sums_only_positive_values() {
        assert_eq!(sum_positive(&[-3, 4, 0, 7, -1]), 11);
    }

    #[test]
    fn positive_sum_is_zero_when_no_positive_values_exist() {
        assert_eq!(sum_positive(&[-3, 0, -1]), 0);
    }
}
