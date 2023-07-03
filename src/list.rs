#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut res = None;
        for i in v.iter().rev() {
            let mut node = Self::new(*i);
            node.next = res;
            res = Some(Box::new(node));
        }

        res
    }

    pub fn into_vec(&self) -> Vec<i32> {
        let mut res = vec![self.val];

        let mut head = &self.next;
        while let Some(node) = head {
            res.push(node.val);
            head = &node.next;
        }

        res
    }
}
