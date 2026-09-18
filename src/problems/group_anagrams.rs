pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_groups:
            std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            let key = chars.into_iter().collect::<String>();
            //anagram_groups.entry(key).or_insert_with(Vec::new).push(s);
            anagram_groups.entry(key).or_default().push(s);
        }

        //anagram_groups.into_iter().map(|(_, v)| v).collect()
        anagram_groups.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for group in &mut groups {
            group.sort();
        }

        groups.sort();
        groups
    }

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn basic_case() {
        let input = strings(&["eat", "tea", "tan", "ate", "nat", "bat"]);

        let result = Solution::group_anagrams(input);

        let expected = vec![
            strings(&["eat", "tea", "ate"]),
            strings(&["tan", "nat"]),
            strings(&["bat"]),
        ];

        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn single_empty_string() {
        let input = strings(&[""]);

        let result = Solution::group_anagrams(input);

        let expected = vec![strings(&[""])];

        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn single_string() {
        let input = strings(&["a"]);

        let result = Solution::group_anagrams(input);

        let expected = vec![strings(&["a"])];

        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn all_strings_are_anagrams() {
        let input = strings(&["abc", "bca", "cab", "cba"]);

        let result = Solution::group_anagrams(input);

        let expected = vec![strings(&["abc", "bca", "cab", "cba"])];

        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn no_strings_are_anagrams() {
        let input = strings(&["abc", "def", "ghi"]);

        let result = Solution::group_anagrams(input);

        let expected = vec![
            strings(&["abc"]),
            strings(&["def"]),
            strings(&["ghi"]),
        ];

        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn duplicate_strings() {
        let input = strings(&["abc", "abc", "bca"]);

        let result = Solution::group_anagrams(input);

        let expected = vec![strings(&["abc", "abc", "bca"])];

        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn different_character_counts_are_not_anagrams() {
        let input = strings(&["aab", "abb", "aba", "baa"]);

        let result = Solution::group_anagrams(input);

        let expected = vec![
            strings(&["aab", "aba", "baa"]),
            strings(&["abb"]),
        ];

        assert_eq!(normalize(result), normalize(expected));
    }

    #[test]
    fn multiple_empty_strings() {
        let input = strings(&["", "", "a"]);

        let result = Solution::group_anagrams(input);

        let expected = vec![
            strings(&["", ""]),
            strings(&["a"]),
        ];

        assert_eq!(normalize(result), normalize(expected));
    }
}

