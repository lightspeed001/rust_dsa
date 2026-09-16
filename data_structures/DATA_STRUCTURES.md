# rust_dsa
## Data Structures in Rust

**Basic structures**: [Basic Structures](data_structures/DATA_STRUCTURES.md)

- _List (Dynamic Array)_ : Default choice for sequences. Use when you need contiguous memory and fast random access by index. `Vec<T>`
- _LinkedList (Doubly Linked List)_ : Use when you need sonstant time-splitting or merging of lists and frequent insertions/removals at both ends without needing random access. `std::collections::LinkedList<T>`
- _ArrayList (Same as `Vec`)_ : See List 
- _Queue (FIFO)_ : Use for FIFO, such as task scheduling, buffering data streams, or implementing a Breadth-First Search (BFS). `std::collections::VecDeque<T>`
- _Stack (LIFO)_ : Use for LIFO logic. Since `Vec` supports push and pop from the end, it is the standard way to implement undo mechanisms or Depth First Search (DFS).
- _Binary Tree_ : Use when you need to maintain a sorted collection of elements and require logarithms time for insertions, removals and searches. `BTreeMap<K, V` or `BTreeSet<T>`
- _Graph (Adjacency List)_ : Use for reprsenting social networks, social connections or map routing where you need to track back relationships between nodes and their neighbors. `Vec<Vec<usize>>` or `HashMap<K, Vec<V>>`

**Additional structures**: [Additional Structures](data_structures/DATA_STRUCTURES.md) 

- _Trie_: Autocomplete, spell check. Rust std lib equiv `None` (use `HashMap` manually).
- _Disjoint Set_: Kruskal's algorithm, network connectivity. Rust std lib equiv `None` (use `HashMap` manually).
- _Bloom Filter_: Probabilistic membership tests. Rust std lib equiv `None` (use `bitvec` crate).
- _Min-Heap_: Priority queues (min-first). Rust std lib equiv `BinaryHeap` (max-heap by default).
- _LRU Cache_: Caching with eviction policies. Rust std lib equiv `None` (use `HashMap` + `LinkedHashMap` from `linked_hash_map` crate).
- _Skip List_: Fast search/insert/delete. Rust std lib equiv `None` (use `BTreeMap` or `HashMap`);
- _Fenwick Tree_: Prefix sums, range queries. Rust std lib equiv `None`(use `None` directly) 

<!--
Advanced Data Structures
- B-Trees (for databases)
- Suffix Trees/ Suffix Arrays (for string matching)
- Quadtrees/Octrees (for spacial partitioning)
- Persistent Data Structures (immutable versions of structures)
-->
