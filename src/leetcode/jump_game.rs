//! <https://leetcode.com/problems/jump-game/>
//!
//! You are given an integer array nums. You are initially positioned at
//! the array's first index, and each element in the array represents your
//! maximum jump length at that position.
//!
//! Return true if you can reach the last index, or false otherwise.
//!
//! Constraints:
//! - 1 <= nums.length <= 10^4
//! - 0 <= nums[i] <= 10^5
pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut furthest = *nums.first().expect("Constraint: nums.length >= 1") as usize;
        let end = nums.len() - 1;

        for (i, &dist) in nums.iter().enumerate() {
            if i > furthest {
                return false;
            }
            furthest = std::cmp::max(furthest, i + dist as usize);
            if furthest >= end {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn testcase1() {
        assert_eq!(Solution::can_jump(vec![0]), true);
    }

    #[test]
    fn testcase2() {
        assert_eq!(Solution::can_jump(vec![2, 0, 0]), true);
    }
}
