pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        todo!("Implement Two Sum II")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn uses_one_based_indices() {
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(Solution::two_sum(vec![-5, -2, 1, 4, 9], 2), vec![2, 4]);
    }

    #[test]
    fn duplicate_values() {
        assert_eq!(Solution::two_sum(vec![1, 2, 3, 3, 5], 6), vec![3, 4]);
    }

    #[test]
    fn two_elements_only() {
        assert_eq!(Solution::two_sum(vec![1, 2], 3), vec![1, 2]);
    }

    #[test]
    fn target_is_negative() {
        assert_eq!(Solution::two_sum(vec![-10, -7, -3, 1, 8], -10), vec![2, 3]);
    }

    #[test]
    fn solution_near_edges() {
        assert_eq!(Solution::two_sum(vec![1, 4, 6, 8, 11, 20], 21), vec![1, 6]);
    }

    #[test]
    fn repeated_negative_values() {
        assert_eq!(Solution::two_sum(vec![-4, -4, 0, 3, 9], -8), vec![1, 2]);
    }
}
