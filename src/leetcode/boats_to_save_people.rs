//! <https://leetcode.com/problems/boats-to-save-people/>
//!
//! You are given an array people where people[i] is the weight of the ith person,
//! and an infinite number of boats where each boat can carry a maximum weight of limit.
//! Each boat carries at most two people at the same time, provided the sum of the weight
//! of those people is at most limit.
//!
//! Return the minimum number of boats to carry every given person.
//!
//! Constraints:
//! 1 <= people.length <= 5 * 10^4
//! 1 <= people[i] <= limit <= 3 * 10^4
pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_unstable();

        let mut count = people.len() as i32;
        let mut i = 0;
        let mut j = people.len() - 1;

        while i < j {
            if people[i] + people[j] <= limit {
                i += 1;
                count -= 1;
            }
            j -= 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let actual = Solution::num_rescue_boats(vec![1, 2], 3);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example2() {
        let actual = Solution::num_rescue_boats(vec![3, 2, 2, 1], 3);
        let expected = 3;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example3() {
        let actual = Solution::num_rescue_boats(vec![3, 5, 3, 4], 5);
        let expected = 4;
        assert_eq!(actual, expected);
    }
}
