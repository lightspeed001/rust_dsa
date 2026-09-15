#[derive(Debug)]
struct Stack<T> {
data: Vec<T>,
}

impl<T> Stack<T> {
fn new() -> Self {
Stack { data: Vec::new()}
}

fn push(&mut self, item: T) {
self.data.push(item);
}

fn pop(&mut self) -> Option<T> {
self.data.pop()
}

}

fn main() {
let mut stack = Stack::new();
stack.push(1);
stack.push(2);
println!(":?", stack.pop()); // Some(2)
}

