use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let (p, q) = (p.unwrap().borrow().val, q.unwrap().borrow().val);
    let mut root = root;
    while let Some(node) = root {
        if p > node.borrow().val && q > node.borrow().val {
            root = node.borrow().right.clone();
        } else if p < node.borrow().val && q < node.borrow().val {
            root = node.borrow().left.clone();
        } else {
            return Some(node.clone());
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = match lowest_common_ancestor(
            TreeNode::from_array(&[6, 2, 8, 0, 4, 7, 9, i32::MIN, i32::MIN, 3, 5]),
            TreeNode::from_array(&[2]),
            TreeNode::from_array(&[8]),
        ) {
            Some(t) => t.borrow().val,
            None => i32::MIN,
        };
        assert_eq!(res, 6);
    }

    #[test]
    fn test2() {
        let res = match lowest_common_ancestor(
            TreeNode::from_array(&[6, 2, 8, 0, 4, 7, 9, i32::MIN, i32::MIN, 3, 5]),
            TreeNode::from_array(&[2]),
            TreeNode::from_array(&[4]),
        ) {
            Some(t) => t.borrow().val,
            None => i32::MIN,
        };
        assert_eq!(res, 2);
    }
}
