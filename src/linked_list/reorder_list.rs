use crate::list::ListNode;

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    // not worth the effort for a noob rustacean atm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut res = ListNode::from_vec(vec![1, 2, 3, 4]);
        reorder_list(&mut res);
        assert_eq!(res.unwrap().into_vec(), vec![1, 4, 2, 3])
    }

    #[test]
    fn test2() {
        let mut res = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        reorder_list(&mut res);
        assert_eq!(res.unwrap().into_vec(), vec![1, 5, 2, 4, 3])
    }
}
