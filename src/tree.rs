// use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_array(v: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if v.is_empty() {
            return None;
        }

        let mut root = Self::new(v[0]);
        root.fill(v, 0);

        Some(Rc::new(RefCell::new(root)))
    }

    fn fill(&mut self, vals: &[i32], index: usize) {
        let left_node = index * 2 + 1;
        if left_node < vals.len() && vals[left_node] != i32::MIN {
            self.left = Some(Rc::new(RefCell::new(Self::new(vals[left_node]))));
            self.left
                .as_ref()
                .unwrap()
                .borrow_mut()
                .fill(vals, left_node);
        }

        let right_node = left_node + 1;
        if right_node < vals.len() && vals[right_node] != i32::MIN {
            self.right = Some(Rc::new(RefCell::new(Self::new(vals[right_node]))));
            self.right
                .as_ref()
                .unwrap()
                .borrow_mut()
                .fill(vals, right_node);
        }
    }

    pub fn into_vec_df(&self) -> Vec<i32> {
        let mut res = vec![];
        self.walk(&mut res);
        res
    }

    fn walk(&self, res: &mut Vec<i32>) {
        if let Some(left) = &self.left {
            left.borrow().walk(res);
        }
        res.push(self.val);
        if let Some(right) = &self.right {
            right.borrow().walk(res);
        }
    }
    //
    // pub fn into_vec_bf(&self) -> Vec<i32> {
    //     let mut res = vec![];
    //     let mut q = VecDeque::new();
    //
    //     // let mut cur = self;
    //     while !q.is_empty() {
    //         let node: Rc<RefCell<TreeNode>> = q.pop_back().unwrap();
    //         res.push(node.borrow().val);
    //         if let Some(n) = node.borrow().left {
    //             q.push_front(n);
    //         }
    //         if let Some(n) = node.borrow().right {
    //             q.push_front(n);
    //         }
    //     }
    //     res
    // }
}
