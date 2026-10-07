use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut h_map = HashMap::new();
        for n in nums.iter() {
            *h_map.entry(*n).or_insert(0) += 1;
        }
        let mut res = Vec::new();
        for h in h_map.iter() {
            res.push((*h.0, *h.1));
        }
        res.sort_by(|&a, &b| a.1.cmp(&b.1));

        let mut top = Vec::new();
        let mut found = 0;
        for i in res.iter().rev() {
            top.push(i.0);
            found += 1;
            if found == k {
                break;
            }
        }
        top
        //todo!("Implement Top K Frequent Elements")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    fn as_set(values: Vec<i32>) -> HashSet<i32> {
        values.into_iter().collect()
    }

    #[test]
    fn basic_case() {
        let result = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        assert_eq!(as_set(result), as_set(vec![1, 2]));
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }

    #[test]
    fn negative_values() {
        let result = Solution::top_k_frequent(vec![-1, -1, -2, -2, -2, -3], 2);
        assert_eq!(as_set(result), as_set(vec![-1, -2]));
    }

    #[test]
    fn all_unique_k_equals_length() {
        let result = Solution::top_k_frequent(vec![4, 5, 6], 3);
        assert_eq!(as_set(result), as_set(vec![4, 5, 6]));
    }

    #[test]
    fn clear_frequency_ordering() {
        let result = Solution::top_k_frequent(vec![5, 5, 5, 5, 4, 4, 4, 3, 3, 2], 3);
        assert_eq!(as_set(result), as_set(vec![5, 4, 3]));
    }
}
