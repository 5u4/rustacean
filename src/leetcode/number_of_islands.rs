//! <https://leetcode.com/problems/number-of-islands/>
//!
//! Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water),
//! return the number of islands.
//!
//! An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically.
//! You may assume all four edges of the grid are all surrounded by water.
//!
//! Constraints:
//!
//! m == grid.length
//! n == grid[i].length
//! 1 <= m, n <= 300
//! grid[i][j] is '0' or '1'.

pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let row_count = grid.len();
        let col_count = grid[0].len();
        let mut visited = vec![vec![false; col_count]; row_count];

        let mut count = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' && !visited[i][j] {
                    Solution::dfs_corrupt_island(&grid, &mut visited, i, j);
                    count += 1;
                }
            }
        }

        count
    }

    fn dfs_corrupt_island(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] != '1' || visited[i][j] {
            return;
        }

        visited[i][j] = true;

        Solution::dfs_corrupt_island(grid, visited, i.saturating_sub(1), j);
        Solution::dfs_corrupt_island(grid, visited, i + 1, j);
        Solution::dfs_corrupt_island(grid, visited, i, j.saturating_sub(1));
        Solution::dfs_corrupt_island(grid, visited, i, j + 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];

        let actual = Solution::num_islands(grid);
        let expected = 1;

        assert_eq!(actual, expected);
    }

    #[test]
    fn example2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        let actual = Solution::num_islands(grid);
        let expected = 3;

        assert_eq!(actual, expected);
    }
}
