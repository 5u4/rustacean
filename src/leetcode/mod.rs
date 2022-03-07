use std::cell::RefCell;
use std::rc::Rc;

pub mod binary_tree_zigzag_level_order_traversal;
pub mod coin_change;
pub mod container_with_most_water;
pub mod generate_parentheses;
pub mod jump_game;
pub mod kth_largest_element_in_an_array;
pub mod median_of_two_sorted_arrays;
pub mod min_stack;
pub mod number_of_islands;
pub mod two_sum;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    /// My self defined function for easier tree construction
    pub fn build_tree_from_vec(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        TreeNode::build_root_tree_node(&values, 0)
    }

    fn build_root_tree_node(
        values: &Vec<Option<i32>>,
        root_index: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(Some(value)) = values.get(root_index) {
            let mut root = TreeNode::new(*value);

            root.left = TreeNode::build_root_tree_node(values, root_index * 2 + 1);
            root.right = TreeNode::build_root_tree_node(values, root_index * 2 + 2);

            return Some(Rc::new(RefCell::new(root)));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add more tests

    #[test]
    fn build_simple_tree() {
        let root = TreeNode::build_tree_from_vec(vec![Some(0), Some(1), Some(2)]).unwrap();
        assert_eq!(root.borrow().val, 0);
    }
}
