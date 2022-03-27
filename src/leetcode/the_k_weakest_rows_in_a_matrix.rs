//! <https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/>
//!
//! You are given an m x n binary matrix mat of 1's (representing soldiers) and 0's (representing civilians).
//! The soldiers are positioned in front of the civilians. That is, all the 1's will appear to the left of all
//! the 0's in each row.
//!
//! A row i is weaker than a row j if one of the following is true:
//! - The number of soldiers in row i is less than the number of soldiers in row j.
//! - Both rows have the same number of soldiers and i < j.
//!
//! Return the indices of the k weakest rows in the matrix ordered from weakest to strongest.
//!
//! Constraints:
//!
//! m == mat.length
//! n == mat[i].length
//! 2 <= n, m <= 100
//! 1 <= k <= m
//! matrix[i][j] is either 0 or 1.

pub struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut rows = mat
            .iter()
            .enumerate()
            .map(|(idx, row)| (idx, row.iter().filter(|&&x| x == 1).count()))
            .collect::<Vec<_>>();

        rows.sort_by(|a, b| a.1.cmp(&b.1));

        rows.iter()
            .take(k as usize)
            .map(|&(idx, _)| idx as i32)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::k_weakest_rows(
                vec![
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 1]
                ],
                3
            ),
            vec![2, 0, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::k_weakest_rows(
                vec![
                    vec![1, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![1, 0, 0, 0],
                    vec![1, 0, 0, 0]
                ],
                2
            ),
            vec![0, 2]
        );
    }
}
