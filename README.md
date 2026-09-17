# rust_dsa
## DSAs in Rust

- **Data Structures**: [data_structures.md](/data_structures/DATA_STRUCTURES.md) 
- **Algorithms**: [algorithms.md](/algorithms/ALGOS.md)

### Rust Primary Standard Library Structures :rescue_worker_helmet:

**1. Sequences (Ordered Collections)**

- `Vec<T>`: A growable array (dynamic array)
- `VecDeque<T>`: A double-ended queue (deque) implemented as a ring buffer.
- `LinkedList<T>`: A doubly linked list (rarely used due to poor cache locality).

**2. Maps (Key-Value Stores)**

- `HashMap<K, V>`: A hash map (unordered, uses `Hash` trait for keys).
- `BTreeMap<K, V>`: A map based on B-tree (ordered by keys).

**3. Sets (Unique Elements)**

- `HashSet<T>`: A hash set (unordered, uses `Hash` trait).
- `BTreeSet<T>`: A set based on B-Tree (ordered).

**4. Miscelleneous**

- `BinaryHeap<T>`: A priority queue (max heap by default).
- `String`: A UTF-8 encoded, growable
- `DeString`/`PathBuf`: Platform-specific string and path handling

**5. Other Useful Types**

- `Cow<'a, T>'`(Copy on Wire): An enum for borrowed or owned data.
- `Box<T>`: A heap-allocated smart pointer.
- `Rc<T>`/`Arc<T>`: Reference-counting smart pointers (single-threaded and thread-safe, respectively).

**Example Usage**

```rust
use std::borrow::Cow;
use std::cmp::Reverse;
use std::collections::{HashMap, VecDeque, BinaryHeap, Vec, LinkedList, BTreeMap, HashSet, BTreeSet, BinaryHeap, Box};
use std::rc::Rc;
use std::sync::Arc;

fn main() {
  
// Vec (dynamic array)
let mut vec = Vec::new();
vec.push(1);
vec.push(2);

// HashMap (key-value store)
let mut map = HashMap::new();
map.insert("key", "value");

// VecDeque (double-ended queue)
let mut deque = VecDeque::new();
deque.push_front(1);
deque.push_back(2);

// BinaryHeap (priority queue)
let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(2);

// Linked List
// LinkedList is a doubly linked list, useful when you need efficient insertions and removals at both ends.

let mut list = LinkedList::new();
list.push_back(1);
list.push_back(2);
list.push_front(6);
 for val in list {
   println!("{:?}", val); // 0, 1, 2
 }

 // useful when you need to iterate over entries in a specific order.
 // B-Treemap: sorted map of keys to values
 let mut scores = BTreeMap::new();
 scores.insert("Alice", 10);
 scores.insert("Bob", 20);

 if let Some(score) = scores.get("Alice") {
   println!("Alice: {score}"); // Iterates in key order
 }

 for (name, score) in &scores {
   println!("{name}: {score}"); // Iterates in key order
 }

 // Hashset: unique values, optimized fir fast lookup
 let mut languages = HashSet::new();
 languages.insert("Rust");
 languages.insert("Python");
 languages.insert("Rust"); // Duplicate: ignored

 println!("Has Rust: {}", langugaes.contains("Rust"));

 //BTreeSet: unique values kept in sorted order
 let mut numbers = BTreeSet::new();
 numbers.insert(30);
 numbers.insert(10);
 numbers.insert(20);

 for number in &numbers {
    println!("{number}"); // 10, 20, 30
 }

 // BinaryHeap: priority queue; largest item is returned first
 let mut heap = BinaryHeap::new();
 heap.push(10);
 heap.push(30);
 heap.push(20);

 while let Some(value) = heap.pop() {
    println!("Largest next: {value}"); // 30, 20, 10
 }

 // A min-heap can be made with Reverse
 let mut min_heap = BinaryHeap::new();
 min_heap.push(Reverse(30));
 min_heap.push(Reverse(10));
 min_heap.push(Reverse(20));

 while let Some(Reverse(value)) = min_heap.pop() {
    println!("Smallest next: {value}"); // 10, 20, 30
 }

 // Cow ("clone on write"): either borrowed or owned data
 fn make_uppercase(input: Cow<'_, str) -> Cow<'_, str> {
    if input.chars().all(char::is_uppercase) {
       input // No allocation; remains borrowed or owned
    } else {
       Cow::Owned(input.to_uppercase()) // Allocates only when needed
    }
 }

 let borrowed = make_uppercase(Cow::Borrowed("hello"));
 let already_uppercase = make_uppercase(Cow::Borrowed("HELLO"));

 println!("{borrowed}");
 println!("{already_uppercase}");

 // Box: stores a value on the heap and owns it
 let boxed_number = Box::new(42);
 println!("Boxed number: {}", *boxed_number);

 // Box is useful for recursive types
 enum List {
    Cons{i32, Box<List>},
    Nil,
 }

 let list = List::Cons(
    1,
    Box::new(List::Cons(2, Box::new(List::Nil))),
 );

 // Rc: single-threaded reference counting
 let shared_text = Rc::new(String::from("shared"));

 let first_owner = Rc::clone(&shared_text);
 let second_owner = Rc::clone(&shared_text);

 println!("{first_owner}");
 println!("{second_owner}");
 println!("Rc owners: {}", Rc::strong_count(&shared_text));

 // Arc: thread-safe reference counting
 let shared_number = Arc::new(100);

 let thread_number = Arc::clone(&shared_number);
 let handle = std::thread::spawn(move || {
    println!("Number from another thread: {thread_number}");
 });

 handle.join().unwrap();
 println!("Number in main thread: {shared_number}");
}
```

