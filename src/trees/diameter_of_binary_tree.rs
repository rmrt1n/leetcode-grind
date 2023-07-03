use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            None => (0, 0),
            Some(node) => {
                let (l_diameter, l_max) = dfs(node.borrow().left.clone());
                let (r_diameter, r_max) = dfs(node.borrow().right.clone());
                (
                    1 + l_diameter.max(r_diameter),
                    l_max.max(r_max.max(l_diameter + r_diameter)),
                )
            }
        }
    }

    dfs(root).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = diameter_of_binary_tree(TreeNode::from_array(&[1, 2, 3, 4, 5]));
        assert_eq!(res, 3);
    }

    #[test]
    fn test2() {
        let res = diameter_of_binary_tree(TreeNode::from_array(&[1, 2]));
        assert_eq!(res, 1);
    }
}
