use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node<T> {
value: T,
next: Option<Rc<RefCell<Node<T>>>>,
prev: option<Weak<RefCell<Node<T>>>>
}

#[derive(Debug)]
struct LinkedLink<T> {
fn new() -> Self {
LinkedList {head: None, tail: None }
}

fn push_front(&mut self, value: T) {
let new_node = Rc::new(RefCell::new(Node {
value,
next: self.head.take(),
prev: None,
}));

if let Some(ref old_head) = self.head {
old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
} else {
self.tail = Some(Rc::downgrade(&new_node));
}
self.head = Some(new_node);
}
}

fn main() {
let mut list = LinkedList::new();
list.push_front(1);
list.push_front(2);
println!("{:?}", list); // LinkedList { head: Some(...), tail: Some(...)}
}

