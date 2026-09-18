// Fenwick Tree (Binary Indexed Tree)
// Efficiently computes sums and updates in logarithmic time.

#[derive(Debug)]
struct FenwickTree {
tree: Vec<i32>,
}

impl FenwickTree {
fn new(size: usize) -> Self {
FenwickTree {
tree: vec![0; size + 1],
}
}

fn update(&mut self, mut index: usize, delta: i32) {
index += 1; // 1 based indexing
while index < self.tree.len() {
self.tree[index] += delta;
index += index & index.wrapping_neg(); // Lowest set bit
}
}

fn query(&self, mut index: usize) -> i32 {
index += 1; // 1 based indexing
let mut sum = 0;
while index > 0 {
sum += self.tree[index];
index-= index & index.wrapping_neg() // Lowest set bit
}

sum
}
}

fn main() {
let mut fenwick = FenwickTree::new(5);
fenwick.update(0, 1);
fenwick.update(1, 2);
fenwick.update(2, 3);
println!("{}", fenwick.query(2)); // 6 (1 + 2 + 3)
}

