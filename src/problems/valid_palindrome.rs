pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let ch: Vec<char> = s.chars().collect();
        let mut begin: usize = 0;
        let mut end = ch.len() - 1;

        while begin < end {
            if !ch[begin].is_ascii_alphanumeric() {
                begin += 1;
                continue;
            }

            if !ch[end].is_ascii_alphanumeric() {
                end -= 1;
                continue;
            }

            let c1 = ch[begin].to_ascii_lowercase();
            let c2 = ch[end].to_ascii_lowercase();
            if c1 != c2 {
                return false;
            }
            begin += 1;
            end -= 1;
        }

        true
        //todo!("Implement Valid Palindrome")
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
