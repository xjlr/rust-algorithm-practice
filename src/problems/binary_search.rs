pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len();

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        -1
        //todo!("Implement Binary Search")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_middle_element() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn returns_minus_one_when_missing() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn finds_first_element() {
        assert_eq!(Solution::search(vec![1, 3, 5, 7, 9], 1), 0);
    }

    #[test]
    fn finds_last_element() {
        assert_eq!(Solution::search(vec![1, 3, 5, 7, 9], 9), 4);
    }

    #[test]
    fn single_element_found() {
        assert_eq!(Solution::search(vec![5], 5), 0);
    }

    #[test]
    fn single_element_missing() {
        assert_eq!(Solution::search(vec![5], 3), -1);
    }

    #[test]
    fn handles_negative_values() {
        assert_eq!(Solution::search(vec![-10, -7, -3, 0, 4, 8], -7), 1);
    }

    #[test]
    fn target_smaller_than_all_values() {
        assert_eq!(Solution::search(vec![2, 4, 6, 8], 1), -1);
    }

    #[test]
    fn target_larger_than_all_values() {
        assert_eq!(Solution::search(vec![2, 4, 6, 8], 10), -1);
    }
}
