mod list;
pub use list::{ListNode, List};

#[cfg(test)]
mod test{
    use crate::{ListNode, List};
    #[test]
    fn test_push_front() {
        let mut list = List::new();
        let node = ListNode::new(2);
        let node2 = ListNode::new(3);
        list.push_front(Box::new(node));
        list.push_front(Box::new(node2));
        assert_eq!(Some(&2), list.last());
    }

    #[test]
    fn test_push_back() {
        let mut list = List::new();
        let node = ListNode::new(3);
        let node2 = ListNode::new(4);
        list.push_back(Box::new(node));
        list.push_back(Box::new(node2));
        assert_eq!(Some(&4), list.last());
    }

    #[test]
    fn test_push_pop() {
        let mut list = List::new();
        let node = ListNode::new(3);
        let node2 = ListNode::new(4);
        list.push_back(Box::new(node));
        list.push_back(Box::new(node2));
        list.pop_back();
        assert_eq!(1, list.len());
    }
}