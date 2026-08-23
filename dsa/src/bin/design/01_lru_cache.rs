// LeetCode Problem 146: LRU Cache (Least Recently Used)
// Approach: HashMap + Doubly Linked List (Index-based arena for 100% safe O(1) ops)
// Time: O(1) for both get() and put() | Space: O(capacity)
// Link: https://leetcode.com/problems/lru-cache/

use std::collections::HashMap;

struct Node {
    key: i32,
    val: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
    free_indices: Vec<usize>,
    head: usize, // Dummy head
    tail: usize, // Dummy tail
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let cap = capacity as usize;
        let mut nodes = Vec::with_capacity(cap + 2);
        // Create dummy head (0) and dummy tail (1)
        nodes.push(Node {
            key: 0,
            val: 0,
            prev: None,
            next: Some(1),
        });
        nodes.push(Node {
            key: 0,
            val: 0,
            prev: Some(0),
            next: None,
        });

        Self {
            capacity: cap,
            map: HashMap::with_capacity(cap),
            nodes,
            free_indices: Vec::new(),
            head: 0,
            tail: 1,
        }
    }

    fn remove_node(&mut self, idx: usize) {
        let prev = self.nodes[idx].prev.unwrap();
        let next = self.nodes[idx].next.unwrap();
        self.nodes[prev].next = Some(next);
        self.nodes[next].prev = Some(prev);
    }

    fn insert_to_head(&mut self, idx: usize) {
        let first = self.nodes[self.head].next.unwrap();
        self.nodes[idx].prev = Some(self.head);
        self.nodes[idx].next = Some(first);
        self.nodes[self.head].next = Some(idx);
        self.nodes[first].prev = Some(idx);
    }

    fn move_to_head(&mut self, idx: usize) {
        self.remove_node(idx);
        self.insert_to_head(idx);
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&idx) = self.map.get(&key) {
            let val = self.nodes[idx].val;
            self.move_to_head(idx);
            val
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&idx) = self.map.get(&key) {
            self.nodes[idx].val = value;
            self.move_to_head(idx);
        } else {
            if self.map.len() >= self.capacity {
                // Evict LRU from tail
                let lru_idx = self.nodes[self.tail].prev.unwrap();
                let lru_key = self.nodes[lru_idx].key;
                self.remove_node(lru_idx);
                self.map.remove(&lru_key);
                self.free_indices.push(lru_idx);
            }

            let new_idx = if let Some(idx) = self.free_indices.pop() {
                self.nodes[idx] = Node {
                    key,
                    val: value,
                    prev: None,
                    next: None,
                };
                idx
            } else {
                let idx = self.nodes.len();
                self.nodes.push(Node {
                    key,
                    val: value,
                    prev: None,
                    next: None,
                });
                idx
            };

            self.insert_to_head(new_idx);
            self.map.insert(key, new_idx);
        }
    }
}

fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1); // returns 1
    cache.put(3, 3); // evicts key 2
    assert_eq!(cache.get(2), -1); // returns -1 (not found)
    cache.put(4, 4); // evicts key 1
    assert_eq!(cache.get(1), -1); // returns -1 (not found)
    assert_eq!(cache.get(3), 3); // returns 3
    assert_eq!(cache.get(4), 4); // returns 4

    println!("All test cases passed for LRU Cache (O(1) Get and Put)!");
}
