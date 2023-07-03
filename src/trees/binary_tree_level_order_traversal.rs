use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::tree::TreeNode;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut q = VecDeque::new();

    if let Some(root) = root {
        q.push_front(root);
    }

    while !q.is_empty() {
        let mut level = vec![];

        for _ in 0..q.len() {
            let node = q.pop_back().unwrap();
            level.push(node.borrow().val);

            if let Some(left) = node.borrow().left.clone() {
                q.push_front(left);
            }

            if let Some(right) = node.borrow().right.clone() {
                q.push_front(right);
            };
        }

        res.push(level);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = level_order(TreeNode::from_array(&[3, 9, 20, i32::MIN, i32::MIN, 15, 7]));
        assert_eq!(res, vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn test2() {
        let res = level_order(TreeNode::from_array(&[1]));
        assert_eq!(res, vec![vec![1]]);
    }

    #[test]
    fn test3() {
        let res = level_order(TreeNode::from_array(&[]));
        assert_eq!(res, Vec::<Vec<i32>>::new());
    }
}
