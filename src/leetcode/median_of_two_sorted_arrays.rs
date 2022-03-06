//! <https://leetcode.com/problems/median-of-two-sorted-arrays/>
//!
//! Given two sorted arrays nums1 and nums2 of size m and n respectively,
//! return the median of the two sorted arrays.
//!
//! The overall run time complexity should be O(log (m+n)).
//!
//! Constraints:
//!
//! nums1.length == m
//! nums2.length == n
//! 0 <= m <= 1000
//! 0 <= n <= 1000
//! 1 <= m + n <= 2000
//! -10^6 <= nums1[i], nums2[i] <= 10^6
pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n1 = nums1.len();
        let n2 = nums2.len();

        // Make sure the shorter vector is at first
        if n1 > n2 {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }

        // Each side, there should be (n + m + 1) / 2 elements
        let el_per_side = (n1 + n2 + 1) / 2;

        // Binary search boundary (we do binary search on nums1)
        let mut start = 0;
        let mut end = n1;

        while start <= end {
            // Do binary search on nums1
            let p1 = (start + end) >> 1;

            // Compute the nums2 partition based on #nums1 == #nums2
            let p2 = el_per_side - p1;

            // Get the boundaries
            let v1_left = if p1 == 0 {
                std::i32::MIN
            } else {
                nums1[p1 - 1]
            };
            let v1_right = if p1 == n1 { std::i32::MAX } else { nums1[p1] };
            let v2_left = if p2 == 0 {
                std::i32::MIN
            } else {
                nums2[p2 - 1]
            };
            let v2_right = if p2 == n2 { std::i32::MAX } else { nums2[p2] };

            // Found the result
            if v1_left <= v2_right && v1_right >= v2_left {
                let is_even_length = (n1 + n2) % 2 == 0;

                return if is_even_length {
                    f64::from(v1_left.max(v2_left) + v1_right.min(v2_right)) / 2.0
                } else {
                    f64::from(v1_left.max(v2_left))
                };
            }

            if v1_left > v2_right {
                end = p1 - 1;
            }
            // v2_left > v1_right
            else {
                start = p1 + 1;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);
    }

    #[test]
    fn example2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);
    }

    #[test]
    fn example3() {
        let nums1 = vec![0, 0];
        let nums2 = vec![0, 0];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 0.0);
    }

    #[test]
    fn example4() {
        let nums1 = vec![];
        let nums2 = vec![1];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 1.0);
    }

    #[test]
    fn example5() {
        let nums1 = vec![2];
        let nums2 = vec![];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);
    }
}
