use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let val = node.borrow().val;
                let acc = if val >= max { 1 } else { 0 };
                acc + dfs(node.borrow().left.clone(), max.max(val))
                    + dfs(node.borrow().right.clone(), max.max(val))
            }
        }
    }

    let val = root.as_ref().unwrap().borrow().val;
    dfs(root, val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = good_nodes(TreeNode::from_array(&[3, 1, 4, 3, i32::MIN, 1, 5]));
        assert_eq!(res, 4);
    }

    #[test]
    fn test2() {
        let res = good_nodes(TreeNode::from_array(&[3, 3, i32::MIN, 4, 2]));
        assert_eq!(res, 3);
    }

    #[test]
    fn test3() {
        let res = good_nodes(TreeNode::from_array(&[1]));
        assert_eq!(res, 1);
    }
}
