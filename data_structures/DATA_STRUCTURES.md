# rust_dsa
## Data Structures in Rust

**Basic structures**: [Basic Structures](basic/)

-
**Additional structures**: [Additional Structures](additional/) 

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
