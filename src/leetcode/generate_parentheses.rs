//! <https://leetcode.com/problems/generate-parentheses/>
//!
//! Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
//!
//! Constraints:
//!
//! 1 <= n <= 8

pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n <= 0 {
            return Vec::new();
        }

        let mut results = Vec::new();

        Self::backtrack(&mut results, &mut "".to_string(), n as usize, n as usize);

        results
    }

    fn backtrack(
        results: &mut Vec<String>,
        curr: &mut String,
        open_left: usize,
        close_left: usize,
    ) {
        if open_left == 0 && close_left == 0 {
            results.push(curr.clone());
            return;
        }

        if open_left > 0 {
            curr.push('(');
            Self::backtrack(results, curr, open_left - 1, close_left);
            curr.pop();
        }

        if close_left > open_left {
            curr.push(')');
            Self::backtrack(results, curr, open_left, close_left - 1);
            curr.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let actual = Solution::generate_parenthesis(3);
        let expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];

        assert_eq!(actual, expected);
    }

    #[test]
    fn example2() {
        let actual = Solution::generate_parenthesis(1);
        let expected = vec!["()"];

        assert_eq!(actual, expected);
    }
}
