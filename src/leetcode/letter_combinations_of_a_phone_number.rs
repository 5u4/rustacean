//! <https://leetcode.com/problems/letter-combinations-of-a-phone-number/>
//!
//! Given a string containing digits from 2-9 inclusive, return all possible letter combinations
//! that the number could represent. Return the answer in any order.
//!
//! A mapping of digit to letters (just like on the telephone buttons) is given below. Note that
//! 1 does not map to any letters.
//!
//!
//! Constraints:
//!
//! 0 <= digits.length <= 4
//! digits[i] is a digit in the range ['2', '9'].
pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res = Vec::new();
        if digits.is_empty() {
            return res;
        }

        let d = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];
        let indices: Vec<u32> = digits
            .chars()
            .map(|x| x.to_digit(10).unwrap() - 2)
            .collect();

        Solution::backtrack(&mut res, &mut String::new(), &indices, &d, 0);

        res
    }

    fn backtrack(
        res: &mut Vec<String>,
        curr: &mut String,
        indices: &Vec<u32>,
        d: &Vec<Vec<char>>,
        i: usize,
    ) {
        if i >= indices.len() {
            res.push(curr.clone());
            return;
        }

        let index = indices[i] as usize;
        let chars = &d[index];

        for c in chars.iter() {
            curr.push(*c);
            Solution::backtrack(res, curr, indices, d, i + 1);
            curr.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new()
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a", "b", "c"]
        );
    }
}
