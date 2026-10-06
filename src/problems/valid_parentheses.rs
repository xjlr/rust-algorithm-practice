pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for p in s.chars() {
            match p {
                '{' | '[' | '(' => stack.push(p),

                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }

                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }

                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }

                _ => return false,
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_parentheses() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn all_bracket_types() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn nested_brackets() {
        assert!(Solution::is_valid("{[()]}".to_string()));
    }

    #[test]
    fn wrong_closing_order() {
        assert!(!Solution::is_valid("(]".to_string()));
    }

    #[test]
    fn crossing_pairs_are_invalid() {
        assert!(!Solution::is_valid("([)]".to_string()));
    }

    #[test]
    fn unmatched_opening_bracket() {
        assert!(!Solution::is_valid("((".to_string()));
    }

    #[test]
    fn unmatched_closing_bracket() {
        assert!(!Solution::is_valid(")".to_string()));
    }

    #[test]
    fn starts_with_closing_bracket() {
        assert!(!Solution::is_valid("]{}".to_string()));
    }

    #[test]
    fn deeply_nested_valid_expression() {
        assert!(Solution::is_valid("(({{[[]]}}))".to_string()));
    }

    #[test]
    fn valid_prefix_but_invalid_suffix() {
        assert!(!Solution::is_valid("()[]{".to_string()));
    }
}
