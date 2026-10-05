pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        todo!("Implement Valid Palindrome")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_palindrome_with_spaces_and_punctuation() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
    }

    #[test]
    fn basic_non_palindrome() {
        assert!(!Solution::is_palindrome("race a car".to_string()));
    }

    #[test]
    fn empty_after_filtering_is_palindrome() {
        assert!(Solution::is_palindrome(" ".to_string()));
    }

    #[test]
    fn single_character() {
        assert!(Solution::is_palindrome("a".to_string()));
    }

    #[test]
    fn digits_can_participate() {
        assert!(Solution::is_palindrome("1a2a1".to_string()));
    }

    #[test]
    fn digits_can_make_string_non_palindrome() {
        assert!(!Solution::is_palindrome("0P".to_string()));
    }

    #[test]
    fn mixed_case_is_ignored() {
        assert!(Solution::is_palindrome("RaceCar".to_string()));
    }

    #[test]
    fn punctuation_only() {
        assert!(Solution::is_palindrome(".,!?;:".to_string()));
    }

    #[test]
    fn letters_and_digits_with_noise() {
        assert!(Solution::is_palindrome("No 'x' in Nixon".to_string()));
    }

    #[test]
    fn almost_palindrome() {
        assert!(!Solution::is_palindrome("abca".to_string()));
    }
}
