use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::tree::TreeNode;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut q = VecDeque::new();

    if let Some(root) = root {
        q.push_front(root);
    }

    while !q.is_empty() {
        let mut node = Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        }));

        for _ in 0..q.len() {
            node = q.pop_back().unwrap();

            if let Some(left) = node.borrow().left.clone() {
                q.push_front(left);
            }

            if let Some(right) = node.borrow().right.clone() {
                q.push_front(right);
            }
        }

        res.push(node.borrow().val);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = right_side_view(TreeNode::from_array(&[1, 2, 3, i32::MIN, 5, i32::MIN, 4]));
        assert_eq!(res, vec![1, 3, 4]);
    }

    #[test]
    fn test2() {
        let res = right_side_view(TreeNode::from_array(&[1, i32::MIN, 3]));
        assert_eq!(res, vec![1, 3]);
    }

    #[test]
    fn test3() {
        let res = right_side_view(TreeNode::from_array(&[]));
        assert_eq!(res, vec![]);
    }
}
