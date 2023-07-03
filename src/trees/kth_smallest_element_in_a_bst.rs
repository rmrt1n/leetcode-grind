use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    fn into_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(node) => into_vec(node.borrow().left.clone())
                .into_iter()
                .chain(vec![node.borrow().val])
                .chain(into_vec(node.borrow().right.clone()))
                .collect(),
        }
    }
    into_vec(root)[k as usize - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = kth_smallest(
            TreeNode::from_array(&[5, 3, 6, 2, 4, i32::MIN, i32::MIN, 1]),
            3,
        );
        assert_eq!(res, 3)
    }

    #[test]
    fn test2() {
        let res = kth_smallest(TreeNode::from_array(&[3, 1, 4, i32::MIN, 2]), 1);
        assert_eq!(res, 1)
    }
}
