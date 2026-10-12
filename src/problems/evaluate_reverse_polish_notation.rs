pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        for token in tokens {
            match token.parse::<i32>() {
                Ok(value) => stack.push(value),
                Err(_) => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();

                    let result = match token.as_str() {
                        "+" => lhs + rhs,
                        "-" => lhs - rhs,
                        "*" => lhs * rhs,
                        "/" => lhs / rhs,
                        _ => unreachable!(),
                    };

                    stack.push(result);
                }
            }
        }

        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn tokens(values: &[&str]) -> Vec<String> {
        values.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn addition_and_multiplication() {
        assert_eq!(
            Solution::eval_rpn(tokens(&["2", "1", "+", "3", "*"])),
            9
        );
    }

    #[test]
    fn division() {
        assert_eq!(
            Solution::eval_rpn(tokens(&["4", "13", "5", "/", "+"])),
            6
        );
    }

    #[test]
    fn larger_expression() {
        assert_eq!(
            Solution::eval_rpn(tokens(&[
                "10", "6", "9", "3", "+", "-11", "*", "/", "*",
                "17", "+", "5", "+"
            ])),
            22
        );
    }

    #[test]
    fn negative_result() {
        assert_eq!(
            Solution::eval_rpn(tokens(&["3", "5", "-"])),
            -2
        );
    }

    #[test]
    fn single_value() {
        assert_eq!(
            Solution::eval_rpn(tokens(&["42"])),
            42
        );
    }
}
