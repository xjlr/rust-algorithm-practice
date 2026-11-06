pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low: i32 = 0;
        let mut high: i32 = nums.len() as i32 - 1;
        while low <= high {
            let middle = low + (high - low) / 2;
            
            if nums[middle as usize] == target {
                return  middle as i32;
            }
            // left side
            if nums[low as usize] <= nums[middle as usize] {
                if nums[low as usize] <= target && target < nums[middle as usize] {
                    high = middle - 1
                } else {
                    low = middle + 1
                }
            } else {
                if target <= nums[high as usize] && target > nums[middle as usize] {
                    low = middle + 1
                } else {
                    high = middle - 1
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn basic_case_found() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn basic_case_not_found() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn single_element_found() {
        assert_eq!(Solution::search(vec![1], 1), 0);
    }

    #[test]
    fn single_element_not_found() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }

    #[test]
    fn not_rotated() {
        assert_eq!(Solution::search(vec![1, 2, 3, 4, 5], 4), 3);
    }

    #[test]
    fn pivot_near_start() {
        assert_eq!(Solution::search(vec![5, 1, 2, 3, 4], 5), 0);
    }

    #[test]
    fn pivot_near_end() {
        assert_eq!(Solution::search(vec![2, 3, 4, 5, 1], 1), 4);
    }
}
