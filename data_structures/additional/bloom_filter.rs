// Bloom Filter
// A probabalistic data stucture for membership tests (may have false positives).

use std::collections::BitVec;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug)]
struct BloomFIlter {
bit_array: BitVec,
num_hashes: usize,
}

impl BloomFilter {
fn new(size: usize, num_hashes: usize) -> Self {
BloomFilter {
bit_array: BitVec::from_elem(size, false),
num_hashes,
}
}

fn insert<T: Hash>(&mut self, item: T) {
let mut hasher = DefaultHasher::new();
for i in 0..self.num_hashes {
hasher.write_usize(i);
item.hash(&mut hasher);
let hash = hasher.finish() as usize % self.bit_array.len();
self.bit_array.set(hash, true);
}
}

fn contains<T: Hash> (&mut self, item: T) {
let mut hasher = DefaultHasher::new();
for i in 0..self.num_hashes {
hasher.write_usize(1);
item.hash(&mut hasher);
let hash = hasher.finish() as usize % self.bit_array.len();
if !self.bit_array[hash]{
return false;
}
}
true
}


}

fn main() {
let mut bloom = BloomFilter::new(100, 3);
bloom.insert("hello");
println!("{}", bloom.contains("hello")); // true
println!("{}", bloom.contains("world")); // false (or true, due to false positives)
}

