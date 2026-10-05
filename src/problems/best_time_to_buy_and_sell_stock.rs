pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut l = 0;
        let mut r = 1;
        let mut max_p = 0;
        while r < prices.len() {
            if prices[r] > prices[l] {
                max_p = max_p.max(prices[r] - prices[l]);
            } else {
                l = r;
            }
            r += 1;
        }

        max_p
        //todo!("Implement Best Time to Buy and Sell Stock")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_profit_case() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn no_profit_possible() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn increasing_prices() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::max_profit(vec![5]), 0);
    }

    #[test]
    fn two_elements_profit() {
        assert_eq!(Solution::max_profit(vec![1, 10]), 9);
    }

    #[test]
    fn two_elements_no_profit() {
        assert_eq!(Solution::max_profit(vec![10, 1]), 0);
    }

    #[test]
    fn best_buy_is_not_first_minimum_seen_at_end() {
        assert_eq!(Solution::max_profit(vec![3, 2, 6, 1, 4]), 4);
    }

    #[test]
    fn repeated_prices() {
        assert_eq!(Solution::max_profit(vec![2, 2, 2, 5, 5]), 3);
    }

    #[test]
    fn profit_after_large_drop() {
        assert_eq!(Solution::max_profit(vec![10, 9, 1, 8]), 7);
    }
}
