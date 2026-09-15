#[derive(Debug)]
struct List<T> {
data: Vec<T>,
}

impl<T> List<T> {
fn new() -> Self {
List {data: Vec::new()}
}

fn push(&mut self, item: T) {
self.data.push(item);
}

fn pop(&mut self, item: T) {
self.data.pop()
}

fn pop(&mut self) -> Option<T. {
selff.data.pop()
}

fn get(&self, index: usize) -> Ooption<&T> {
self.data.get(index)
}
}

fn main() {
let mut list = List::new();
list.push(1);
list.push(2);
println!("{:?}", list); // List { data: [1, 2]}
}

