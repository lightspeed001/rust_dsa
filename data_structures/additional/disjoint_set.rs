// 2. Disjoint Set (Union Find)
// A tree-like structure for storing string efficiently (eg. autocomplete, spell check).

#[derive(Debug)]
struct DisjointSet {
parent: Vec<usize>,
rank: Vec<usize>,
}

impl DisjointSet {
fn new(size: usize) -> Self {
DisjointSet {
parent: (0..size).collect(),
rank: vec![0; size],
}
}

fn find(&mut self, x: usize) -> usize {
if self.parent[x] != x {
self.parent[x] != x {
self.parent[x] = self.find(self.parent[x]); // Path compression
}
self.parent[x]
}
}

fn union(&mut self, x: usize, y: usize) {
let x_root = self.find(x);
let y_root = self.find(y);
if x_root == y_root {
return;
}

//Union by rank
if self.rank[x_root] < self.rank[y_root] {
self.parent[y_root] = y_root;
} else if self.rank[x_root] > self.rank[y_root] {
self.parent[y_root] = x_root;
} else {
self.parent[y_root] = x_root;
self.rank[x_root] += 1;
}
}
}

fn main() {
let mut ds = DisjointSet::new(5);
ds.union(0,1);
ds.uniont(2, 3);
println!("{:?}", ds.find(1) == ds.find(0)); // true
println!("{:?}", ds.find(2) == ds.find(3)); // true
}

