use std::{collections::HashMap, rc::Rc};

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Rc<ListNode>>,
}

pub struct LRUCache {
    capacity: i32,
    len: i32, 
    map: HashMap<i32, Rc<ListNode>>,
    head: Rc<ListNode>,
    cur: Rc<ListNode>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let mut dummy = Rc::new(ListNode { val: 0, next: None });
        Self { capacity, len: 0, map: HashMap::new(), head: dummy, cur: Rc::clone(&dummy) }
    }

    pub fn get(&self, key: i32) -> i32 {
        0
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let new_node = Rc::new(ListNode { val: value, next: None });
        self.cur.next = Some(new_node);
        self.map.insert(key, self.cur);
        self.cur = self.cur.next.take().unwrap();

        self.len += 1;
        if self.len > self.capacity {
            self.head = self.head.next.take().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut c = LRUCache::new(2);
        c.put(1, 1);
        c.put(2, 2);

        let res = c.get(1);
        assert_eq!(res, 1);

        c.put(3, 3);
        let res = c.get(2);
        assert_eq!(res, -1);

        c.put(4, 4);
        let res = c.get(1);
        assert_eq!(res, -1);

        let res = c.get(3);
        assert_eq!(res, 3);
        let res = c.get(4);
        assert_eq!(res, 4);
    }
}
