// 1. Trie (Prefix Tree)
// A tree-like structure for storing string efficiently (eg. autocomplete, spell check).

use std::collections::HashMap;

#[derive(Debug, Default)]
struct TrieNode {
children: HashMap<char, TrieNode>,
is_end_of_word: bool,
}
```

#[derive(Debug, Default)]
struct Trie {
root: TrieNode,
}

impl Trie {
fn new() -> Self {
Trie {
root: TrieNode::default(),
}
}

fn insert(&mut self, word: &str) {
let mut current = &mut self.root;
for ch in word.chars() {
current = current.children.entry(ch).or_default();

}
current.is_end_of_word = true;
}

fn search(&self, word: &str) -> bool {
let mut current = &self.root;
for ch in word.chars() {
if let Some(node) = current.children.get(&ch) {
current = node;
} else {
return false;
}
}
current.is_end_of_word
}
}

fn main() {
let mut trie = Trie::new();
trie.insert("apple");
println("{}", trie.search("apple")); //true
println("{}", trie.search("app")); //false
}

