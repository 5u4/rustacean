//! <https://leetcode.com/problems/container-with-most-water/>
//!
//! Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i, ai).
//! n vertical lines are drawn such that the two endpoints of the line i is at (i, ai) and (i, 0).
//! Find two lines, which, together with the x-axis forms a container, such that the container contains
//! the most water.
//!
//! **Notice** that you may not slant the container.
//!
//! Constraints:
//!
//! n == height.length
//! 2 <= n <= 10^5
//! 0 <= height[i] <= 10^4
pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut local_max = 0;

        while i < j {
            let local_min_height = height[i].min(height[j]);
            local_max = local_max.max(local_min_height * (j - i) as i32);

            while i < j && height[i] <= local_min_height {
                i += 1;
            }

            while i < j && height[j] <= local_min_height {
                j -= 1;
            }
        }

        local_max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
    }
}
