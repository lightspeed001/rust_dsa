// 5. LRU Cache (Least Recently Used)
// A cache that evicts the least recently used item when full.

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
struct LRUCache<K, V> {
capacity: usize,
cache: HashMap<K, V>,
order: Vec<K>,
}

impl<K: Eq + Hash + Clone, V> LRUCache<K, V> {
fn new(capacity: usize) -> Self {
LRUCache {
capacity: usize,
cache: HashMap<K, V>,
order: Vec<K>,
}
}

fn get(&mut self, key: &K) -> Option<&V> {
if let Some(value) = self.cache.get(key) {
// Move key to the end (most recently used)
if let Some(pos) = self.order.iter().position(|x| x == key) {
self.order.remove(pos);
}
self.order.push(key.clone());
Some(value)
} else {
None
}
}

fn put(&mut self, key: K, value: V) {
if self.cache.contains_key(&key) {
// Update existing key
if let Some(pos) = self.order.iter().position(|x| x == &key) {
self.order.remove(pos);
}
} else if self.cache.len() >= self.capacity {
// Evict least recently used
if let Some(lru_key) = self.order.remove(0) {
self.cache.remove(&lru_key);
}
}
self.cache.insert(key.clone(), value);
self.order.push(key);
}
}

fn main() {
let mut cache = LRUCache::new(2);
cache.put("a", 1);
cache.put("b", 2);
println!("{:?}", cache.get(&"a")); // Some(1)
cache.put("c", 3); // Evicts "b"
println!("{:?}", cache.get(&"b")); // None
}

