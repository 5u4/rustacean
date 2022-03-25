//! <https://leetcode.com/problems/two-city-scheduling/>
//!
//! A company is planning to interview 2n people. Given the array costs where costs[i] = [aCosti, bCosti],
//! the cost of flying the ith person to city a is aCosti, and the cost of flying the ith person to city b is bCosti.
//!
//! Return the minimum cost to fly every person to a city such that exactly n people arrive in each city.
//!
//! Constraints:
//!
//! 2 * n == costs.length
//! 2 <= costs.length <= 100
//! costs.length is even.
//! 1 <= aCosti, bCosti <= 1000
pub struct Solution;

impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut costs = costs;
        costs.sort_by(|a, b| (a[0] - a[1]).partial_cmp(&(b[0] - b[1])).unwrap());
        let n = costs.len() / 2;
        let mut res = 0;
        for i in 0..n {
            res += costs[i][0] + costs[i + n][1];
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let actual = Solution::two_city_sched_cost(vec![
            vec![10, 20],
            vec![30, 200],
            vec![400, 50],
            vec![30, 20],
        ]);
        let expected = 110;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example2() {
        let actual = Solution::two_city_sched_cost(vec![
            vec![259, 770],
            vec![448, 54],
            vec![926, 667],
            vec![184, 139],
            vec![840, 118],
            vec![577, 469],
        ]);
        let expected = 1859;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example3() {
        let actual = Solution::two_city_sched_cost(vec![
            vec![515, 563],
            vec![451, 713],
            vec![537, 709],
            vec![343, 819],
            vec![855, 779],
            vec![457, 60],
            vec![650, 359],
            vec![631, 42],
        ]);
        let expected = 3086;
        assert_eq!(actual, expected);
    }
}
