use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, left: i64, right: i64) -> bool {
        match node {
            None => true,
            Some(node)
                if !((node.borrow().val as i64) > left && (node.borrow().val as i64) < right) =>
            {
                false
            }
            Some(node) => {
                let val = node.as_ref().borrow().val as i64;
                dfs(node.borrow().left.clone(), left, val)
                    && dfs(node.borrow().right.clone(), val, right)
            }
        }
    }
    dfs(root, i64::MIN, i64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = is_valid_bst(TreeNode::from_array(&[2, 1, 3]));
        assert!(res);
    }

    #[test]
    fn test2() {
        let res = is_valid_bst(TreeNode::from_array(&[5, 1, 4, i32::MIN, i32::MIN, 3, 6]));
        assert!(!res);
    }
}
