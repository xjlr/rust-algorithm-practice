use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut low = 0;
        let mut high = 0;
        let mut max_len = 0;
        let mut set: HashSet<char> = HashSet::new();
        let substr: Vec<char> = s.chars().collect();
        while high < substr.len() {
            if set.contains(&substr[high]) {
                set.remove(&substr[low]);
                low += 1;
            } else {
                set.insert(substr[high]);
                high += 1;
                let diff = high - low;
                max_len = max_len.max(diff);
            }
        }
        max_len as i32
        //todo!("Implement Longest Substring Without Repeating Characters")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn all_same_characters() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn mixed_repetition() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn empty_string() {
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }

    #[test]
    fn single_character() {
        assert_eq!(Solution::length_of_longest_substring("a".to_string()), 1);
    }

    #[test]
    fn repeated_character_after_gap() {
        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
    }

    #[test]
    fn repeated_prefix_character() {
        assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);
    }

    #[test]
    fn includes_space_character() {
        assert_eq!(
            Solution::length_of_longest_substring("a b c a".to_string()),
            3
        );
    }

    #[test]
    fn all_unique_characters() {
        assert_eq!(
            Solution::length_of_longest_substring("abcdef".to_string()),
            6
        );
    }
}
