//use std::num::IntErrorKind;

pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        let mut intervals = intervals;
        intervals.sort_unstable();

        let mut result = Vec::new();
        let mut current = intervals[0].clone();

        for interval in intervals.into_iter().skip(1) {
            if interval[0] <= current[1] {
                current[1] = current[1].max(interval[1]);
            } else {
                result.push(current);
                current = interval;
            }
        }

        result.push(current);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn basic_case() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn touching_intervals_merge() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
    }

    #[test]
    fn already_disjoint() {
        assert_eq!(
            Solution::merge(vec![vec![1, 2], vec![3, 4], vec![5, 6]]),
            vec![vec![1, 2], vec![3, 4], vec![5, 6]]
        );
    }

    #[test]
    fn nested_intervals() {
        assert_eq!(
            Solution::merge(vec![vec![1, 10], vec![2, 3], vec![4, 8]]),
            vec![vec![1, 10]]
        );
    }

    #[test]
    fn unsorted_input() {
        assert_eq!(
            Solution::merge(vec![vec![8, 10], vec![1, 3], vec![2, 6]]),
            vec![vec![1, 6], vec![8, 10]]
        );
    }
}
