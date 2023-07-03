use crate::list::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (Some(l1), Some(l2)) => {
            if l1.val < l2.val {
                return Some(Box::new(ListNode {
                    val: l1.val,
                    next: merge_two_lists(l1.next, Some(l2)),
                }));
            }
            Some(Box::new(ListNode {
                val: l2.val,
                next: merge_two_lists(Some(l1), l2.next),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = match merge_two_lists(
            ListNode::from_vec(vec![1, 2, 4]),
            ListNode::from_vec(vec![1, 3, 4]),
        ) {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test2() {
        let res = match merge_two_lists(ListNode::from_vec(vec![]), ListNode::from_vec(vec![])) {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![]);
    }

    #[test]
    fn test3() {
        let res = match merge_two_lists(ListNode::from_vec(vec![]), ListNode::from_vec(vec![0])) {
            Some(v) => v.into_vec(),
            None => vec![],
        };
        assert_eq!(res, vec![0]);
    }
}
