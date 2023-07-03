use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    fn is_sametree(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                a.borrow().val == b.borrow().val
                    && is_sametree(a.borrow().left.clone(), b.borrow().left.clone())
                    && is_sametree(a.borrow().right.clone(), b.borrow().right.clone())
            }
            _ => false,
        }
    }

    match (root, sub_root) {
        (_, None) => true,
        (None, _) => false,
        (Some(root), Some(sub_root)) => {
            if is_sametree(Some(root.clone()), Some(sub_root.clone())) {
                return true;
            }
            is_subtree(root.borrow().left.clone(), Some(sub_root.clone()))
                || is_subtree(root.borrow().right.clone(), Some(sub_root))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = is_subtree(
            TreeNode::from_array(&[3, 4, 5, 1, 2, 0]),
            TreeNode::from_array(&[4, 1, 2]),
        );
        assert!(res);
    }
}
