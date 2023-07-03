use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            p == q
                && is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                && is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = is_same_tree(
            TreeNode::from_array(&[1, 2, 3]),
            TreeNode::from_array(&[1, 2, 3]),
        );
        assert!(res);
    }

    #[test]
    fn test2() {
        let res = is_same_tree(
            TreeNode::from_array(&[1, 2, 1]),
            TreeNode::from_array(&[1, 1, 2]),
        );
        assert!(!res);
    }

    #[test]
    fn test3() {
        let res = is_same_tree(
            TreeNode::from_array(&[1, 2]),
            TreeNode::from_array(&[0, 1, 2]),
        );
        assert!(!res);
    }
}
