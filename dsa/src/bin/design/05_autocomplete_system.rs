// LeetCode Problem 642: Design Search Autocomplete System
// Approach: Trie with Hot Sentences List / Frequency Map
// Time: O(p + N log 3) for input() | Space: O(total sentence characters)
// Link: https://leetcode.com/problems/design-search-autocomplete-system/

use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    counts: HashMap<String, i32>,
}

pub struct AutocompleteSystem {
    root: TrieNode,
    cur_sentence: String,
}

impl AutocompleteSystem {
    pub fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
        let mut sys = Self {
            root: TrieNode::default(),
            cur_sentence: String::new(),
        };

        for (s, t) in sentences.into_iter().zip(times.into_iter()) {
            sys.insert(&s, t);
        }
        sys
    }

    fn insert(&mut self, sentence: &str, count: i32) {
        let mut curr = &mut self.root;
        for ch in sentence.chars() {
            curr = curr.children.entry(ch).or_default();
            *curr.counts.entry(sentence.to_string()).or_insert(0) += count;
        }
    }

    pub fn input(&mut self, c: char) -> Vec<String> {
        if c == '#' {
            // End of sentence input: commit to history
            let sentence = self.cur_sentence.clone();
            self.insert(&sentence, 1);
            self.cur_sentence.clear();
            return vec![];
        }

        self.cur_sentence.push(c);
        let mut curr = &self.root;

        for ch in self.cur_sentence.chars() {
            if let Some(next) = curr.children.get(&ch) {
                curr = next;
            } else {
                return vec![];
            }
        }

        // Collect top 3 candidates sorted by:
        // 1) Higher frequency (-count)
        // 2) ASCII alphabetical order (sentence)
        let mut candidates: Vec<(&String, &i32)> = curr.counts.iter().collect();
        candidates.sort_unstable_by(|a, b| {
            b.1.cmp(a.1).then_with(|| a.0.cmp(b.0))
        });

        candidates
            .into_iter()
            .take(3)
            .map(|(s, _)| s.clone())
            .collect()
    }
}

fn main() {
    let sentences = vec![
        "i love you".to_string(),
        "island".to_string(),
        "ironman".to_string(),
        "i love leetcode".to_string(),
    ];
    let times = vec![5, 3, 2, 2];

    let mut auto = AutocompleteSystem::new(sentences, times);

    // Typing 'i'
    assert_eq!(
        auto.input('i'),
        vec![
            "i love you".to_string(),
            "island".to_string(),
            "i love leetcode".to_string(),
        ]
    );

    // Typing ' '
    assert_eq!(
        auto.input(' '),
        vec![
            "i love you".to_string(),
            "i love leetcode".to_string(),
        ]
    );

    // Typing 'a' -> no suggestions matching "i a"
    assert_eq!(auto.input('a'), Vec::<String>::new());

    // Typing '#' -> finishes sentence "i a"
    assert_eq!(auto.input('#'), Vec::<String>::new());

    println!("All test cases passed for Design Autocomplete System (Trie + Frequency Ranking)!");
}
