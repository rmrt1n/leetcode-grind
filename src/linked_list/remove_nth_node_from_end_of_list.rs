use crate::list::ListNode;

pub fn nc_remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    // copy (clone) head into dummy.next, head unchanged
    let mut dummy = Box::new(ListNode {
        val: 0,
        next: head.clone(),
    });
    // left = mut pointer of dummy, right = move value of head here, head is now invalid
    let (mut left, mut right) = (dummy.as_mut(), head);

    let mut n = n;
    while n > 0 && right.is_some() {
        right = right.unwrap().next;
        n -= 1;
    }

    while let Some(r) = right {
        // left now points to a mut pointer of left.next
        left = left.next.as_mut().unwrap();
        right = r.next;
    }

    // left.next now points to left.next.next
    // let dropped = "left.next.take()", left.next is now None, and imaginary var 'dropped' now
    // contains left.next
    // left.next = dropped.unwrap().next.take(), dropped.next is now None, and left.next is
    // dropped.next. weird explanation, hopefully I understand when I read this again in the future
    left.next = left.next.take().unwrap().next.take();

    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = match nc_remove_nth_from_end(ListNode::from_vec(vec![1, 2, 3, 4, 5]), 2) {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![1, 2, 3, 5]);
    }

    #[test]
    fn test2() {
        let res = match nc_remove_nth_from_end(ListNode::from_vec(vec![1, 2]), 1) {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![1]);
    }
}
