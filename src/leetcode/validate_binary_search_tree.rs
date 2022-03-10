//! <https://leetcode.com/problems/validate-binary-search-tree/>
//!
//! Given the root of a binary tree, determine if it is a valid binary
//! search tree (BST).
//!
//! A valid BST is defined as follows:
//!
//! - The left subtree of a node contains only nodes with keys less
//!     than the node's key.
//! - The right subtree of a node contains only nodes with keys greater
//!     than the node's key.
//! - Both the left and right subtrees must also be binary search trees.
//!  
//! Constraints:
//!
//! The number of nodes in the tree is in the range [1, 104].
//! -2^31 <= Node.val <= 2^31 - 1
use super::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::is_valid_bst_with_contraint(&root, None, None)
    }

    fn is_valid_bst_with_contraint(
        root: &Option<Rc<RefCell<TreeNode>>>,
        min: Option<i32>,
        max: Option<i32>,
    ) -> bool {
        if root.is_none() {
            return true;
        }

        let root = root.as_deref().unwrap();
        let val = root.borrow().val;

        if let Some(min_val) = min {
            if val <= min_val {
                return false;
            }
        }

        if let Some(max_val) = max {
            if val >= max_val {
                return false;
            }
        }

        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();

        Solution::is_valid_bst_with_contraint(&left, min, Some(val))
            && Solution::is_valid_bst_with_contraint(&right, Some(val), max)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let root = TreeNode::build_tree_from_vec(vec![Some(2), Some(1), Some(3)]);
        assert_eq!(Solution::is_valid_bst(root), true)
    }

    #[test]
    fn example2() {
        let root = TreeNode::build_tree_from_vec(vec![
            Some(5),
            Some(1),
            Some(4),
            None,
            None,
            Some(3),
            Some(6),
        ]);
        assert_eq!(Solution::is_valid_bst(root), false)
    }

    #[test]
    fn example3() {
        let root = TreeNode::build_tree_from_vec(vec![Some(1), Some(1)]);
        assert_eq!(Solution::is_valid_bst(root), false)
    }

    #[test]
    fn example4() {
        let root = TreeNode::build_tree_from_vec(vec![
            Some(32),
            Some(26),
            Some(47),
            Some(19),
            None,
            None,
            Some(56),
            None,
            Some(27),
        ]);
        // Explanation: The root node's value is 5 but its right child's value is 4.
        assert_eq!(Solution::is_valid_bst(root), false)
    }
}
