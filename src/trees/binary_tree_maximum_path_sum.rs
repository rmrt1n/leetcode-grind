use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match node {
            None => (-1001, -1001),
            Some(node) => {
                let (l_max, l_sum) = dfs(node.borrow().left.clone());
                let (r_max, r_sum) = dfs(node.borrow().right.clone());
                let val = node.borrow().val;

                (
                    l_max.max(r_max).max(val + l_sum.max(0) + r_sum.max(0)),
                    val.max(val + l_sum).max(val + r_sum),
                )
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
        let res = max_path_sum(TreeNode::from_array(&[1, 2, 3]));
        assert_eq!(res, 6);
    }

    #[test]
    fn test2() {
        let res = max_path_sum(TreeNode::from_array(&[
            -10,
            9,
            20,
            i32::MIN,
            i32::MIN,
            15,
            7,
        ]));
        assert_eq!(res, 42);
    }

    #[test]
    fn test3() {
        let res = max_path_sum(TreeNode::from_array(&[-3]));
        assert_eq!(res, -3);
    }
}
