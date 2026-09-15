use std::collections::HashMap;

#[derive(Debug)]
struct Graph<T> {
adjacency_list: HashMap<T, Vec<T>>,
}

impl<T: std::hash::Hash + Eq + Clone> Graph<T> {
fn new() -> Self {
Graph {
adjacency_list: HashMap::new(),
}
}

fn add_edge(&mut self, from: T, to: T) {
self.adjacency_list
.entry(from.clone())
.or_insert_with(Vec::new)
.push(to);
}
}

fn main() {
let mut graph = Graph:: new();
graph.add_edge("A", "B");
graph.add_edge("A", "C");
println!("{:?}", graph); // Graph {adjacency_list: {"A": ["B", "C"]}}
}

