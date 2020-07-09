use core::ptr::NonNull;

#[derive(Debug)]
pub struct ListNode<T> {
    next: Option<NonNull<ListNode<T>>>,
    prev: Option<NonNull<ListNode<T>>>,
    value: T,
}


#[derive(Debug)]
pub struct List<T>{
    count: i32,
    first: Option<NonNull<ListNode<T>>>,
    last: Option<NonNull<ListNode<T>>>,
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Self {
        ListNode{
            next: None,
            prev: None,
            value: val,
        }
    }

    pub fn into_value(self: Box<Self>) -> T{
        self.value
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List{
            count: 0,
            first: None,
            last: None,
        }
    }

    pub fn first(&self) -> Option<&T>{
       unsafe{
            self.first.as_ref().map(|node| &node.as_ref().value)
       } 
    }

    pub fn last(&self) -> Option<&T>{
        unsafe{
            self.last.as_ref().map(|node| &node.as_ref().value)
        }
    }

    pub fn len(self) -> i32{
        self.count
    }

    /// Adds the given node to the front of the list.
    pub fn push_front(&mut self, mut node: Box<ListNode<T>>){
        unsafe{
            node.next = self.first;
            // let node = Some(Box::into_raw_non_null(node));
            let node = Some(Box::leak(node).into());
            match self.first {
                None => self.last = node,
                Some(first) => (*first.as_ptr()).prev = node,
            }
            self.first = node;
            self.count +=1;
        }
    }

    /// Removes and returns the node at the front of the list.
    pub fn pop_front(&mut self) -> Option<Box<ListNode<T>>> {
        self.first.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.first = node.next;

            match self.first {
                None => self.last = None,
                Some(first) => (*first.as_ptr()).next = None,
            }
            

            self.count -=1;
            node
        })        
    }

    /// Adds the given node to the back of the list.
    pub fn push_back(&mut self, mut node: Box<ListNode<T>>){
        unsafe{
            node.prev = self.last;
            let node = Some(Box::leak(node).into());
            match self.last {
                None => self.first = node,
                Some(last) => (*last.as_ptr()).next = node,
            }
            self.last = node;
            self.count +=1;
        }
    }

    /// Removes and returns the node at the back of the list.
    pub fn pop_back(&mut self) -> Option<Box<ListNode<T>>> {
        self.last.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.last = node.prev;

            match self.last {
                None => self.first = None,
                Some(last) => (*last.as_ptr()).next = None,
            }

            self.count -=1;
            node
        })        
    }

   


}