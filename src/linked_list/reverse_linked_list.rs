use crate::list::ListNode;

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut node = None;
    while let Some(next) = head {
        node = Some(Box::new(ListNode {
            val: next.val,
            next: node,
        }));
        head = next.next;
    }
    node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = match reverse_list(ListNode::from_vec(vec![1, 2, 3, 4, 5])) {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test2() {
        let res = match reverse_list(ListNode::from_vec(vec![1, 2])) {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![2, 1]);
    }
}
