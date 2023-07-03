use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    println!("{:#?}", root);
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        match root {
            None => (true, 0),
            Some(node) => {
                let (l_balanced, l_max) = dfs(node.borrow().left.clone());
                let (r_balanced, r_max) = dfs(node.borrow().right.clone());
                let balanced = l_balanced && r_balanced && l_max.abs_diff(r_max) <= 1;
                (balanced, 1 + l_max.max(r_max))
            }
        }
    }

    dfs(root).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = is_balanced(TreeNode::from_array(&[3, 9, 20, 15, 7]));
        assert!(res);
    }

    // #[test]
    // fn test2() {
    //     let res = is_balanced(TreeNode::from_array(&[1,2,2,3,3,4,4]));
    //     assert!(!res);
    // }

    #[test]
    fn test3() {
        let res = is_balanced(TreeNode::from_array(&[]));
        assert!(res);
    }
}
