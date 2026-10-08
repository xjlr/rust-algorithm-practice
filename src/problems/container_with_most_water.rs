pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max = 0;
        while left < right {
            let smaller = height[left].min(height[right]);
            let area = smaller * (right as i32 - left as i32);
            max = area.max(max);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max
        //todo!("Implement Container With Most Water")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn basic_case() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn two_elements() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn increasing_heights() {
        assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5]), 6);
    }

    #[test]
    fn decreasing_heights() {
        assert_eq!(Solution::max_area(vec![5, 4, 3, 2, 1]), 6);
    }

    #[test]
    fn equal_heights() {
        assert_eq!(Solution::max_area(vec![4, 4, 4, 4]), 12);
    }
}
