use std::collections::{HashMap};

/// TrieNode struct represents a node in a Trie data structure
///
/// # Properties
/// * `is_end` - A boolean value that indicates if the node is the end of a word
/// * `children` - A HashMap that stores the children of the node
///
struct TrieNode {
    is_end: bool,
    children: HashMap<String, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            is_end: false,
            children: HashMap::default(),
        }
    }
}


/// Trie struct represents a Trie data structure
///
/// # Properties
/// * `root` - A TrieNode that represents the root of the Trie
///
/// # Methods
/// * `new` - Creates a new Trie instance
/// * `insert` - Inserts a word into the Trie
/// * `contains` - Checks if a word is in the Trie
/// * `starts_with` - Checks if a word starts with a prefix in the Trie
///
/// # Examples
/// ```
/// use crate::poc_dsa_trie::Trie;
///
/// let mut my_trie = Trie::new();
/// my_trie.insert("Hello");
/// assert_eq!(my_trie.contains("Hello"), true);
/// assert_eq!(my_trie.starts_with("He"), true);
/// ```
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new()
        }
    }

    pub fn insert(&mut self, word: &str) -> String {
        let mut curr = &mut self.root;

        for c in word.chars() {
            curr = curr.children
                .entry(c.to_string())
                .or_insert_with(TrieNode::new)
        }

        curr.is_end = true;
        word.to_string()
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut curr = &self.root;

        for c in word.chars() {
            match curr.children.get(&c.to_string()) {
                Some(child) => curr = child,
                None => return false,
            }
        }

        curr.is_end
    }

    pub fn starts_with(&mut self, word: &str) -> bool {
        let mut curr = &self.root;

        for c in word.chars() {
            match curr.children.get(&c.to_string()) {
                Some(child) => curr = child,
                None => return false,
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_insert() {
        let mut my_trie = Trie::new();
        let result = my_trie.insert("Hello");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_trie_contains() {
        let mut my_trie = Trie::new();
        my_trie.insert("Hello");
        assert_eq!(my_trie.contains("Hello"), true);
    }

    #[test]
    fn test_trie_starts_with() {
        let mut my_trie = Trie::new();
        my_trie.insert("Hello");
        assert_eq!(my_trie.starts_with("He"), true);
    }
}