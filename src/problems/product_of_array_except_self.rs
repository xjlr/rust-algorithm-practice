pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return nums;
        }

        let mut result: Vec<i32> = vec![1; nums.len()];
        let mut idx: usize = 1;
        let mut prod: i32 = 1;
        while idx < nums.len() {
            prod *= nums[idx - 1];
            result[idx] = prod;
            idx += 1;
        }

        prod = 1;
        idx = nums.len() - 1;

        while idx > 0 {
            prod *= nums[idx];
            result[idx - 1] *= prod;
            idx -= 1;
        }
        result
        //todo!("Implement Product of Array Except Self")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn basic_case() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }

    #[test]
    fn contains_zero() {
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }

    #[test]
    fn two_elements() {
        assert_eq!(Solution::product_except_self(vec![2, 3]), vec![3, 2]);
    }

    #[test]
    fn negative_values() {
        assert_eq!(
            Solution::product_except_self(vec![-1, -2, -3, -4]),
            vec![-24, -12, -8, -6]
        );
    }

    #[test]
    fn multiple_ones() {
        assert_eq!(
            Solution::product_except_self(vec![1, 1, 1, 1]),
            vec![1, 1, 1, 1]
        );
    }
}
