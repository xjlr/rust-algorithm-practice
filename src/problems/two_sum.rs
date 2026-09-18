pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        todo!("Implement Two Sum")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut result: Vec<i32>) -> Vec<i32> {
        result.sort_unstable();
        result
    }

    #[test]
    fn basic_case() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(normalize(result), vec![0, 1]);
    }

    #[test]
    fn pair_in_middle() {
        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(normalize(result), vec![1, 2]);
    }

    #[test]
    fn duplicate_values() {
        let result = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(normalize(result), vec![0, 1]);
    }

    #[test]
    fn negative_numbers() {
        let result = Solution::two_sum(vec![-3, 4, 3, 90], 0);
        assert_eq!(normalize(result), vec![0, 2]);
    }

    #[test]
    fn zero_values() {
        let result = Solution::two_sum(vec![0, 4, 3, 0], 0);
        assert_eq!(normalize(result), vec![0, 3]);
    }

    #[test]
    fn negative_target() {
        let result = Solution::two_sum(vec![-1, -2, -3, -4, -5], -8);
        assert_eq!(normalize(result), vec![2, 4]);
    }

    #[test]
    fn solution_near_end() {
        let result = Solution::two_sum(vec![10, 20, 30, 40, 50], 90);
        assert_eq!(normalize(result), vec![3, 4]);
    }

    #[test]
    fn does_not_reuse_same_element() {
        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_ne!(normalize(result), vec![0, 0]);
    }
}
