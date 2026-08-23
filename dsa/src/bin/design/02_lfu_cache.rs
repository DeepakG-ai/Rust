// LeetCode Problem 460: LFU Cache (Least Frequently Used)
// Approach: HashMap + Frequency Buckets with Doubly Linked Lists
// Time: O(1) for both get() and put() | Space: O(capacity)
// Link: https://leetcode.com/problems/lfu-cache/

use std::collections::HashMap;

struct Node {
    key: i32,
    val: i32,
    freq: usize,
    prev: Option<usize>,
    next: Option<usize>,
}

struct DList {
    head: usize,
    tail: usize,
    len: usize,
}

pub struct LFUCache {
    capacity: usize,
    min_freq: usize,
    key_to_idx: HashMap<i32, usize>,
    freq_to_list: HashMap<usize, DList>,
    nodes: Vec<Node>,
    free_indices: Vec<usize>,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            min_freq: 0,
            key_to_idx: HashMap::new(),
            freq_to_list: HashMap::new(),
            nodes: Vec::new(),
            free_indices: Vec::new(),
        }
    }

    fn create_dlist(&mut self) -> DList {
        let head = self.alloc_node(0, 0, 0);
        let tail = self.alloc_node(0, 0, 0);
        self.nodes[head].next = Some(tail);
        self.nodes[tail].prev = Some(head);
        DList { head, tail, len: 0 }
    }

    fn alloc_node(&mut self, key: i32, val: i32, freq: usize) -> usize {
        if let Some(idx) = self.free_indices.pop() {
            self.nodes[idx] = Node {
                key,
                val,
                freq,
                prev: None,
                next: None,
            };
            idx
        } else {
            let idx = self.nodes.len();
            self.nodes.push(Node {
                key,
                val,
                freq,
                prev: None,
                next: None,
            });
            idx
        }
    }

    fn remove_from_list(&mut self, list_freq: usize, idx: usize) {
        let prev = self.nodes[idx].prev.unwrap();
        let next = self.nodes[idx].next.unwrap();
        self.nodes[prev].next = Some(next);
        self.nodes[next].prev = Some(prev);

        if let Some(list) = self.freq_to_list.get_mut(&list_freq) {
            list.len -= 1;
        }
    }

    fn push_to_head(&mut self, list_freq: usize, idx: usize) {
        if !self.freq_to_list.contains_key(&list_freq) {
            let list = self.create_dlist();
            self.freq_to_list.insert(list_freq, list);
        }
        let list = self.freq_to_list.get_mut(&list_freq).unwrap();
        let first = self.nodes[list.head].next.unwrap();

        self.nodes[idx].prev = Some(list.head);
        self.nodes[idx].next = Some(first);
        self.nodes[list.head].next = Some(idx);
        self.nodes[first].prev = Some(idx);
        list.len += 1;
    }

    fn pop_tail(&mut self, list_freq: usize) -> (i32, usize) {
        let list = self.freq_to_list.get_mut(&list_freq).unwrap();
        let lru_idx = self.nodes[list.tail].prev.unwrap();
        let key = self.nodes[lru_idx].key;
        self.remove_from_list(list_freq, lru_idx);
        (key, lru_idx)
    }

    fn update_frequency(&mut self, idx: usize) {
        let old_freq = self.nodes[idx].freq;
        let new_freq = old_freq + 1;
        self.nodes[idx].freq = new_freq;

        self.remove_from_list(old_freq, idx);
        self.push_to_head(new_freq, idx);

        if old_freq == self.min_freq && self.freq_to_list.get(&old_freq).map(|l| l.len).unwrap_or(0) == 0 {
            self.min_freq += 1;
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if self.capacity == 0 {
            return -1;
        }
        if let Some(&idx) = self.key_to_idx.get(&key) {
            let val = self.nodes[idx].val;
            self.update_frequency(idx);
            val
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        if let Some(&idx) = self.key_to_idx.get(&key) {
            self.nodes[idx].val = value;
            self.update_frequency(idx);
        } else {
            if self.key_to_idx.len() >= self.capacity {
                // Evict from min_freq list
                let (evicted_key, freed_idx) = self.pop_tail(self.min_freq);
                self.key_to_idx.remove(&evicted_key);
                self.free_indices.push(freed_idx);
            }

            let new_idx = self.alloc_node(key, value, 1);
            self.push_to_head(1, new_idx);
            self.key_to_idx.insert(key, new_idx);
            self.min_freq = 1;
        }
    }
}

fn main() {
    let mut lfu = LFUCache::new(2);
    lfu.put(1, 1);
    lfu.put(2, 2);
    assert_eq!(lfu.get(1), 1); // return 1, freq(1)=2
    lfu.put(3, 3); // evicts key 2 (freq(2)=1)
    assert_eq!(lfu.get(2), -1); // returns -1 (not found)
    assert_eq!(lfu.get(3), 3); // return 3, freq(3)=2
    lfu.put(4, 4); // evicts key 1 (freq 1 and 3 are both 2, 1 is LRU)
    assert_eq!(lfu.get(1), -1); // return -1 (not found)
    assert_eq!(lfu.get(3), 3); // return 3
    assert_eq!(lfu.get(4), 4); // return 4

    println!("All test cases passed for LFU Cache (O(1) Get and Put)!");
}
