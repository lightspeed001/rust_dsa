use std::collections::VecDeque;

#[derive(Debug)]
struct Queue<T> {
data: VecDeque<T>,
}

impl<T> Queue<T> {
fn new() -> Self {
Queue {data: VecDeque::new()}
}

fn enqueue(&mut self, item: T) {
self.data.push_back(item);
}

fn dequeue(&mut self) -> Option<T> {
self.data.pop_front()
}
}

fn main() {
let mut queue = Queue::new();
queue.enqueue(1);
queue.enqueue(2);
println!("{:?}", queue.dequeue()); // Some(1)
}

