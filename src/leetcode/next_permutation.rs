//! <https://leetcode.com/problems/next-permutation/>
//!
//! A permutation of an array of integers is an arrangement of its members into a sequence or linear order.
//!
//! - For example, for arr = [1,2,3], the following are considered permutations of arr:
//! [1,2,3], [1,3,2], [3,1,2], [2,3,1].
//!
//! The next permutation of an array of integers is the next lexicographically greater permutation of its integer.
//! More formally, if all the permutations of the array are sorted in one container according to their
//! lexicographical order, then the next permutation of that array is the permutation that follows it in the
//! sorted container. If such arrangement is not possible, the array must be rearranged as the lowest possible order
//! (i.e., sorted in ascending order).
//!
//! - For example, the next permutation of arr = [1,2,3] is [1,3,2].
//! - Similarly, the next permutation of arr = [2,3,1] is [3,1,2].
//! - While the next permutation of arr = [3,2,1] is [1,2,3] because [3,2,1] does not have a lexicographical
//! larger rearrangement.
//!
//! Given an array of integers nums, find the next permutation of nums.
//!
//! The replacement must be in place and use only constant extra memory.
//!
//! Constraints:
//!
//! 1 <= nums.length <= 100
//! 0 <= nums[i] <= 100
pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() as i32 - 2;

        // Find the first decreasing element from the end
        while i >= 0 && nums[i as usize] >= nums[i as usize + 1] {
            i -= 1;
        }

        if i >= 0 {
            let mut j = nums.len() as i32 - 1;

            // Find the number that just larger than the decreasing element
            while j >= 0 && nums[j as usize] <= nums[i as usize] {
                j -= 1;
            }

            nums.swap(i as usize, j as usize);
        }

        // Reverse the decreasing part
        nums[(i + 1) as usize..].reverse();
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example1() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    #[test]
    fn example2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn example3() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }
}
