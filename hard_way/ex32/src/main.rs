use ex32::{ListNode, List};

fn main(){
    let mut list = List::<i32>::new();
    let node = ListNode::new(0);
    let node1 = ListNode::new(1);
    let node2 = ListNode::new(2);
    let node3 = ListNode::new(3);
    let node4 = ListNode::new(4);
    list.push_back(Box::new(node));
    list.push_back(Box::new(node1));
    list.push_back(Box::new(node2));
    list.push_front(Box::new(node3));
    list.push_front(Box::new(node4));
    println!("list: {:?}", list);
    println!("first: {:?}", list.first());
    println!("pop front: {:?}", list.pop_front());
    println!("last: {:?}", list.last());
    println!("pop back: {:?}", list.pop_back());
    println!("list: {:?}", list);
}

