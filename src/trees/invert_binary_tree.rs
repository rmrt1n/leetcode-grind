use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

type TreeRef = Rc<RefCell<TreeNode>>;

pub fn invert_tree(root: Option<TreeRef>) -> Option<TreeRef> {
    root.map(|n| {
        let tmp = n.borrow().left.clone();
        match n.borrow_mut() {
            mut node => {
                node.left = invert_tree(node.right.take());
                node.right = invert_tree(tmp);
            }
        }

        n
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = match invert_tree(TreeNode::from_array(&[4, 2, 7, 1, 3, 6, 9])) {
            Some(t) => t.borrow().into_vec_df(),
            None => vec![],
        };
        assert_eq!(res, vec![9, 7, 6, 4, 3, 2, 1]);
    }

    #[test]
    fn test2() {
        let res = match invert_tree(TreeNode::from_array(&[2, 1, 3])) {
            Some(t) => t.borrow().into_vec_df(),
            None => vec![],
        };
        assert_eq!(res, vec![3, 2, 1]);
    }

    #[test]
    fn test3() {
        let res = match invert_tree(TreeNode::from_array(&[])) {
            Some(t) => t.borrow().into_vec_df(),
            None => vec![],
        };
        assert_eq!(res, vec![]);
    }
}
