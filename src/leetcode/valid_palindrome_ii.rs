//! <https://leetcode.com/problems/valid-palindrome-ii/>
//!
//! Given a string s, return true if the s can be palindrome after deleting at most one character from it.
//!
//! Constraints:
//!
//! 1 <= s.length <= 10^5
//! s consists of lowercase English letters.
pub struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        Solution::is_valid_palindrome_with_deletion(&s.as_bytes(), 0, s.len() - 1, 1)
    }

    fn is_valid_palindrome_with_deletion(s: &[u8], i: usize, j: usize, del_count: usize) -> bool {
        let mut i = i;
        let mut j = j;

        while s[i] == s[j] {
            if i < j {
                i += 1;
                j -= 1;
            } else {
                return true;
            }
        }

        if del_count > 0 {
            Solution::is_valid_palindrome_with_deletion(s, i + 1, j, del_count - 1)
                || Solution::is_valid_palindrome_with_deletion(s, i, j - 1, del_count - 1)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(true, Solution::valid_palindrome("aba".to_string()));
    }

    #[test]
    fn example2() {
        assert_eq!(true, Solution::valid_palindrome("abca".to_string()));
    }

    #[test]
    fn example3() {
        assert_eq!(false, Solution::valid_palindrome("abc".to_string()));
    }
}
