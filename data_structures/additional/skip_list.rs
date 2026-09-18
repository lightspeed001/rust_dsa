// Skip List
// A probabilistic data structure for the fast search, insertion, and deletion (alternatives to balanced trees)

use rand::Rng;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct SkipListNode<T: Ord> {
value: T,
next: Vec<Option<Rc<RefCell<SkipLisyNode<T>>>>>,
}

#[derive(Debug)]
struct SkipList<T: Ord> {
head: Rc<RefCell<SkipListNode<T>>>,
max_level: usize,
rng: rand::rngs::ThreadRng,
}

impl<T: Ord> SkipList<T> {
fn new(max_level: usize) -> Self {
let head = Rc::new(RefCell::new(SkipListNode {
value: T::default(),
next: vec![None, max_level],
}));

SkipList {
head,
max_level,
rng: rand::thread_rng(),
}

}

fn random_level(&mut self) -> usize {
let mut level = 1;
while level < self.max_level && self.rng.gen_bool(0.5) {
level += 1;
}
level
}

fn insert(&mut self, value: T) {
let level = self.random_level();
let mut update = vec![zNone; self.max_level];
let mut current = Rc::clone(&self.head);

for i in (0..self.max_level).rev() {
while let Some(next) = &current.borrow().next[i] {
if next.borrow().value < value {
current = Rc::clone(next);
} else {
break;
}
}
update[i] = Some(Rc::clone(&current));
}
let new_node = Rc::new(RefCell::new(SkipListNode {
value,
next: vec![None, level],
}));

for i in 0..level {
if let Some(prev) = &update[i] {
new_node.borrow_mut().next[i] = prev.borrow().next[i].take();
prev.borrow_mut().next[i] = Some(Rc::clone(&new_node));
}
}
}
}

fn main() {
let mut skip_list = SkipList::new(4);
skip_list.insert(3);
skip_list.insert(1);
skip_list.insert(2);
println!("{:?}", skip_list); // SkipList {head: ..., max_level: 4, ..}
}

