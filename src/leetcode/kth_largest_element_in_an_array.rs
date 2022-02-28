//! <https://leetcode.com/problems/kth-largest-element-in-an-array/>
//!
//! Given an integer array nums and an integer k, return the kth largest element in the array.
//!
//! Note that it is the kth largest element in the sorted order, not the kth distinct element.
//!
//! Constraints:
//!
//! 1 <= k <= nums.length <= 10^4
//! -10^4 <= nums[i] <= 10^4

pub struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len();
        let mut nums = nums.clone();

        let k_idx = k as usize - 1;

        while start < end {
            // Every element before pivot is smaller than pivot.
            let pivot = Solution::partition(&mut nums, start, end - 1);

            // Found the kth element
            if pivot == k_idx {
                return nums[pivot];
            }

            if pivot < k_idx {
                start = pivot + 1;
            } else {
                end = pivot;
            }
        }

        -1
    }

    fn partition(nums: &mut Vec<i32>, start: usize, end: usize) -> usize {
        let pivot = start + (end - start) / 2;
        let target = nums[pivot];

        nums.swap(pivot, end);
        let mut p = start;

        for i in start..end {
            if nums[i] > target {
                nums.swap(i, p);
                p += 1;
            }
        }

        nums.swap(p, end);
        p
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let actual = Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2);
        let expected = 5;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example2() {
        let actual = Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4);
        let expected = 4;
        assert_eq!(actual, expected);
    }
}
