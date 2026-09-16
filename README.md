# rust_dsa
## DSAs in Rust

- **Data Structures**: [data_structures.md](/data_structures/DATA_STRUCTURES.md) 
- **Algorithms**: [algorithms.md](/algorithms/ALGOS.md)

### Rust Primary Standard Library Structures :helmet:

**1. Sequences (Ordered Collections)**

- `Vec<T>`: A growable array (dynamic array)
- `VecDeque<T>`: A double-ended queue (deque) implemented as a ring buffer.
- `LinkedLink<T>`: A doubly linked list (rarely used due to poor cache locality).

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

use std::collections::{HashMap, VecDeque, BinaryHeap};

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
}
```

