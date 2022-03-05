//! <https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/>
//!
//! Given the root of a binary tree, return the zigzag level order traversal
//! of its nodes' values. (i.e., from left to right, then right to left for
//! the next level and alternate between).
//!
//! Constraints:
//!
//! The number of nodes in the tree is in the range [0, 2000].
//! -100 <= Node.val <= 100
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use super::TreeNode;

pub struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        match root {
            None => result,
            Some(root) => {
                let mut queue = VecDeque::new();
                queue.push_back(root);

                let mut forward = true;
                while queue.len() > 0 {
                    let size = queue.len();
                    let mut row = Vec::new();

                    for _ in 0..size {
                        let node = queue.pop_front().unwrap();
                        row.push(node.borrow().val);

                        let left = node.borrow().left.clone();
                        if let Some(left) = left {
                            queue.push_back(left);
                        }

                        let right = node.borrow().right.clone();
                        if let Some(right) = right {
                            queue.push_back(right);
                        }
                    }

                    if !forward {
                        row.reverse();
                    }

                    forward = !forward;
                    result.push(row);
                }

                result
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let root = TreeNode::build_tree_from_vec(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);

        assert_eq!(
            Solution::zigzag_level_order(root),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }

    #[test]
    fn example2() {
        let root = TreeNode::build_tree_from_vec(vec![Some(1)]);
        assert_eq!(Solution::zigzag_level_order(root), vec![vec![1]])
    }

    #[test]
    fn example3() {
        let root = TreeNode::build_tree_from_vec(vec![]);
        assert_eq!(Solution::zigzag_level_order(root), Vec::<Vec<i32>>::new())
    }
}
