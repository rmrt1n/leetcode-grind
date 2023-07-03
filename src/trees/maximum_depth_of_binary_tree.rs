use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    root.map_or(0, |n| {
        1 + max_depth(n.borrow().left.clone()).max(max_depth(n.borrow().right.clone()))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = max_depth(TreeNode::from_array(&[3, 9, 20, 15, 7]));
        assert_eq!(res, 3);
    }

    #[test]
    fn test2() {
        let res = max_depth(TreeNode::from_array(&[1, 2]));
        assert_eq!(res, 2);
    }
}
