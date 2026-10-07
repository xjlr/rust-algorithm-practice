pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums_sort = nums.clone();
        nums_sort.sort_unstable();
        let mut i = 0;
        let mut result = Vec::new();
        while i + 2 < nums_sort.len() {
            if i > 0 && nums_sort[i - 1] == nums_sort[i] {
                i += 1;
                continue;
            }
            let mut j = i + 1;
            let mut k = nums_sort.len() - 1;
            while j < k {
                if j > i + 1 && nums_sort[j - 1] == nums_sort[j] {
                    j += 1;
                    continue;
                }
                let n = nums_sort[i] + nums_sort[j] + nums_sort[k];
                if n == 0 {
                    result.push(vec![nums_sort[i], nums_sort[j], nums_sort[k]]);
                    j += 1;
                    k -= 1;
                } else if n < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
            i += 1
        }
        result
        //todo!("Implement 3Sum")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn normalize(mut values: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for triplet in &mut values {
            triplet.sort_unstable();
        }
        values.sort_unstable();
        values
    }

    #[test]
    fn basic_case() {
        assert_eq!(
            normalize(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])),
            normalize(vec![vec![-1, -1, 2], vec![-1, 0, 1]])
        );
    }

    #[test]
    fn no_solution() {
        assert_eq!(
            normalize(Solution::three_sum(vec![0, 1, 1])),
            Vec::<Vec<i32>>::new()
        );
    }

    #[test]
    fn all_zeroes() {
        assert_eq!(
            normalize(Solution::three_sum(vec![0, 0, 0])),
            vec![vec![0, 0, 0]]
        );
    }

    #[test]
    fn duplicate_values_do_not_duplicate_triplets() {
        assert_eq!(
            normalize(Solution::three_sum(vec![-2, 0, 0, 2, 2])),
            vec![vec![-2, 0, 2]]
        );
    }

    #[test]
    fn multiple_distinct_triplets() {
        assert_eq!(
            normalize(Solution::three_sum(vec![-4, -2, -2, -1, 0, 1, 2, 2, 3])),
            normalize(vec![
                vec![-4, 1, 3],
                vec![-4, 2, 2],
                vec![-2, -1, 3],
                vec![-2, 0, 2],
                vec![-1, 0, 1],
            ])
        );
    }
}
