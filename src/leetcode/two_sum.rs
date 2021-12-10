//! Given an array of integers `nums` and an integer `target`,
//! return _indices of the two numbers such that they add up to `target`_.
//!
//! You may assume that each input would have **_exactly_ one solution**,
//! and you may not use the _same_ element twice.
//!
//! You can return the answer in any order.
//!
//! `Constraints`:
//! * `2 <= nums.length <= 10^4`
//! * `-10^9 <= nums[i] <= 10^9`
//! * `-10^9 <= target <= 10^9`
//! * **Only one valid answer exists.**
//!
//! **Follow-up**: Can you come up with an algorithm that is less than `O(n^2)` time complexity?
//!
//! <https://leetcode.com/problems/two-sum/>
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut composites: HashMap<i32, i32> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            if let Some(index) = composites.get(num) {
                return vec![*index, i as i32];
            }

            let composite = target - num;
            composites.insert(composite, i as i32);
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
