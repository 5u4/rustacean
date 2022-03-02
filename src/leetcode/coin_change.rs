//! <https://leetcode.com/problems/coin-change/>
//!
//! You are given an integer array coins representing coins of different denominations
//! and an integer amount representing a total amount of money.
//!
//! Return the fewest number of coins that you need to make up that amount.
//! If that amount of money cannot be made up by any combination of the coins, return -1.
//!
//! You may assume that you have an infinite number of each kind of coin.
//!
//! Constraints:
//!
//! 1 <= coins.length <= 12
//! 1 <= coins[i] <= 2^31 - 1
//! 0 <= amount <= 10^4

pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let no_sol = amount + 1;

        let mut dp = vec![no_sol; amount as usize + 1];
        dp[0] = 0;

        for coin in coins.iter() {
            for amt in *coin..=amount {
                let without_coin = dp[amt as usize];
                let with_coin = dp[(amt - coin) as usize] + 1;
                dp[(amt as usize)] = without_coin.min(with_coin);
            }
        }

        let last = dp.last().unwrap();
        if *last == no_sol {
            -1
        } else {
            *last
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::coin_change(vec![1], 1), 1);
    }

    #[test]
    fn example5() {
        assert_eq!(Solution::coin_change(vec![1], 2), 2);
    }
}
