use crate::list::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut rem = 0;
    let mut dummy = Box::new(ListNode { val: 0, next: None });
    let mut _res = dummy.as_mut();
    let (mut list1, mut list2) = (l1.as_ref(), l2.as_ref());

    while list1.is_some() && list2.is_some() {
        let sum = list1.unwrap().val + list2.unwrap().val + rem;
        rem = sum / 10;
        _res.next = Some(Box::new(ListNode {
            val: sum % 10,
            next: None,
        }));
        _res = _res.next.as_mut().unwrap();

        list1 = list1.unwrap().next.as_ref();
        list2 = list2.unwrap().next.as_ref();
    }

    while let Some(node) = list1 {
        let sum = list1.unwrap().val + rem;
        rem = sum / 10;
        _res.next = Some(Box::new(ListNode {
            val: sum % 10,
            next: None,
        }));
        _res = _res.next.as_mut().unwrap();

        list1 = node.next.as_ref();
    }

    while let Some(node) = list2 {
        let sum = list2.unwrap().val + rem;
        rem = sum / 10;
        _res.next = Some(Box::new(ListNode {
            val: sum % 10,
            next: None,
        }));
        _res = _res.next.as_mut().unwrap();

        list2 = node.next.as_ref();
    }

    if rem > 0 {
        _res.next = Some(Box::new(ListNode {
            val: rem,
            next: None,
        }));
        _res = _res.next.as_mut().unwrap();
    }

    dummy.next
}

pub fn nc_add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());
    let mut dummy = Box::new(ListNode { val: 0, next: None });
    let mut cur = dummy.as_mut();
    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry > 0 {
        let val1 = l1.map(|n| n.val).unwrap_or_default();
        let val2 = l2.map(|n| n.val).unwrap_or_default();

        let sum = val1 + val2 + carry;
        carry = sum / 10;
        cur.next = Some(Box::new(ListNode {
            val: sum % 10,
            next: None,
        }));
        cur = cur.next.as_mut().unwrap();

        if let Some(node) = l1 {
            l1 = node.next.as_ref()
        }
        if let Some(node) = l2 {
            l2 = node.next.as_ref()
        }
    }

    dummy.next
}

pub fn rec_add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(n1), Some(n2)) => {
            let sum = n1.val + n2.val;
            if sum < 10 {
                Some(Box::new(ListNode {
                    val: sum,
                    next: rec_add_two_numbers(n1.next, n2.next),
                }))
            } else {
                let carry = Some(Box::new(ListNode {
                    val: sum / 10,
                    next: None,
                }));
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: rec_add_two_numbers(rec_add_two_numbers(carry, n1.next), n2.next),
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = match add_two_numbers(
            ListNode::from_vec(vec![2, 4, 3]),
            ListNode::from_vec(vec![5, 6, 4]),
        ) {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![7, 0, 8]);
    }

    #[test]
    fn test2() {
        let res = match add_two_numbers(ListNode::from_vec(vec![0]), ListNode::from_vec(vec![0])) {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![0]);
    }

    #[test]
    fn test3() {
        let res = match add_two_numbers(
            ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]),
            ListNode::from_vec(vec![9, 9, 9, 9]),
        ) {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }

    #[test]
    fn nc_test1() {
        let res = match nc_add_two_numbers(
            ListNode::from_vec(vec![2, 4, 3]),
            ListNode::from_vec(vec![5, 6, 4]),
        ) {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![7, 0, 8]);
    }

    #[test]
    fn nc_test2() {
        let res = match nc_add_two_numbers(ListNode::from_vec(vec![0]), ListNode::from_vec(vec![0]))
        {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![0]);
    }

    #[test]
    fn nc_test3() {
        let res = match nc_add_two_numbers(
            ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]),
            ListNode::from_vec(vec![9, 9, 9, 9]),
        ) {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }

    #[test]
    fn rec_test1() {
        let res = match rec_add_two_numbers(
            ListNode::from_vec(vec![2, 4, 3]),
            ListNode::from_vec(vec![5, 6, 4]),
        ) {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![7, 0, 8]);
    }

    #[test]
    fn rec_test2() {
        let res =
            match rec_add_two_numbers(ListNode::from_vec(vec![0]), ListNode::from_vec(vec![0])) {
                Some(v) => v.into_vec(),
                None => vec![],
            };
        assert_eq!(res, vec![0]);
    }

    #[test]
    fn rec_test3() {
        let res = match rec_add_two_numbers(
            ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]),
            ListNode::from_vec(vec![9, 9, 9, 9]),
        ) {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }
}
