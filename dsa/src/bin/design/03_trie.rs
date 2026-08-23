// LeetCode Problem 208: Implement Trie (Prefix Tree)
// Approach: Tree structure with 26 child pointers per node
// Time: O(L) for insert, search, starts_with | Space: O(total characters)
// Link: https://leetcode.com/problems/implement-trie-prefix-tree/

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end_of_word: bool,
}

#[derive(Default)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a word into the Trie.
    /// Time: O(L) | Space: O(L)
    pub fn insert(&mut self, word: String) {
        let mut curr = &mut self.root;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            curr = curr.children[idx].get_or_insert_with(Box::default);
        }
        curr.is_end_of_word = true;
    }

    /// Returns true if the word is in the Trie.
    /// Time: O(L) | Space: O(1)
    pub fn search(&self, word: String) -> bool {
        let mut curr = &self.root;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            match &curr.children[idx] {
                Some(next) => curr = next,
                None => return false,
            }
        }
        curr.is_end_of_word
    }

    /// Returns true if there is any word in the Trie that starts with the given prefix.
    /// Time: O(L) | Space: O(1)
    pub fn starts_with(&self, prefix: String) -> bool {
        let mut curr = &self.root;
        for b in prefix.bytes() {
            let idx = (b - b'a') as usize;
            match &curr.children[idx] {
                Some(next) => curr = next,
                None => return false,
            }
        }
        true
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert!(trie.search("apple".to_string())); // return True
    assert!(!trie.search("app".to_string())); // return False
    assert!(trie.starts_with("app".to_string())); // return True
    trie.insert("app".to_string());
    assert!(trie.search("app".to_string())); // return True
    assert!(!trie.search("appl".to_string())); // return False
    assert!(!trie.starts_with("b".to_string())); // return False

    println!("All test cases passed for Implement Trie (Insert, Search, StartsWith O(L))!");
}
