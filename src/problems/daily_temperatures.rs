pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0i32; temperatures.len()];
        let mut stack: Vec<(i32, usize)> = Vec::new();

        for (i, &t) in temperatures.iter().enumerate() {
            while let Some(&(top_t, top_i)) = stack.last() {
                if t > top_t {
                    stack.pop();
                    res[top_i] = (i - top_i) as i32;
                } else {
                    break;
                }
            }
            stack.push((t, i));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn basic_case() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn strictly_increasing() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
    }

    #[test]
    fn strictly_decreasing() {
        assert_eq!(
            Solution::daily_temperatures(vec![60, 50, 40, 30]),
            vec![0, 0, 0, 0]
        );
    }

    #[test]
    fn equal_temperatures() {
        assert_eq!(
            Solution::daily_temperatures(vec![70, 70, 70]),
            vec![0, 0, 0]
        );
    }

    #[test]
    fn later_warmer_day() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }
}
