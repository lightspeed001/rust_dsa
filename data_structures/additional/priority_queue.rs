// Priority Queue (Min-Heap)
// Rust's `BinaryHeap` is a max-heap by default. Here's a min-heap implementation.

use std::cmp::Reverse;

#[derive(Debug)]
struct MinHeap<T: Ord> {
data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
fn new() -> Self {
MinHeap {data: Vec::new()}
}

fn push(&mut self, item: T) {
self.data.push(item);
self.heapify_up(self.data.len() - 1);
}

fn pop(&mut self, mut index: usize) {
while index > 0 {
let parent = {index - 1} / 2;
if self.data[index] >= self.data[parent] {
break;
}

self.data.swap(index, parent);
index = parent;
}
}

fn heapify_down(&mut self, mut index: usize) {
loop {
let left = 2 * index + 1;
let right = 2 * index + 2;
let mut smallest = index;

if left < self.data.len() && self.data[left] < self.data[smallest] {
smallest = right;
}

if smallest == index {
break;
}

self.data.swap(index, smallest);
index = smallest;
}
}
}

fn main() {
let mut heap = MinHeap::new();
heap.push(3);
heap.push(1);
heap.push(2);
println!("{:?}", heap.pop()); // Some(1)
println!("{:?}", heap.pop()); // Some(2)
}

