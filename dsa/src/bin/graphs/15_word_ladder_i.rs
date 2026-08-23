// Word Ladder I - LeetCode 127
// Approaches:
//   1) Standard BFS (Unidirectional): -> O(N * M * 26) time | O(N * M) space
//   2) Optimal (Bidirectional BFS): Search from begin and end simultaneously -> O(N^(M/2) * 26) time | O(N * M) space
// Link: https://leetcode.com/problems/word-ladder/
//
// Examples:
//   "hit" -> "hot" -> "dot" -> "dog" -> "cog" (length 5)
//   begin="hit", end="cog", list=[hot,dot,dog,lot,log,cog] -> 5

use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    /// 1. STANDARD BFS (Unidirectional):
    /// Queue stores (current_word, current_steps).
    /// Time: O(N * M * 26) | Space: O(N * M)
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_set: HashSet<String> = word_list.into_iter().collect();

        if !word_set.contains(&end_word) {
            return 0;
        }
        word_set.remove(&begin_word);

        let mut q: VecDeque<(String, i32)> = VecDeque::from([(begin_word, 1)]);

        while let Some((word, steps)) = q.pop_front() {
            if word == end_word {
                return steps;
            }

            let wb: Vec<u8> = word.bytes().collect();
            for i in 0..wb.len() {
                for ch in b'a'..=b'z' {
                    if wb[i] == ch {
                        continue;
                    }
                    let mut candidate = wb.clone();
                    candidate[i] = ch;
                    let cand = String::from_utf8(candidate).unwrap();

                    if word_set.contains(&cand) {
                        word_set.remove(&cand);
                        q.push_back((cand, steps + 1));
                    }
                }
            }
        }
        0
    }

    /// 2. OPTIMAL (Bidirectional BFS):
    /// Maintain forward and backward sets; always expand from the smaller frontier.
    /// Time: O(N^(M/2) * 26) | Space: O(N * M)
    pub fn ladder_length_bidirectional(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> i32 {
        let mut word_set: HashSet<String> = word_list.into_iter().collect();
        if !word_set.contains(&end_word) {
            return 0;
        }

        let mut start_set = HashSet::new();
        let mut end_set = HashSet::new();
        start_set.insert(begin_word);
        end_set.insert(end_word);

        let mut steps = 1;

        while !start_set.is_empty() && !end_set.is_empty() {
            // Always expand the smaller set
            if start_set.len() > end_set.len() {
                std::mem::swap(&mut start_set, &mut end_set);
            }

            let mut next_level = HashSet::new();
            for word in &start_set {
                word_set.remove(word);
            }

            for word in start_set {
                let wb: Vec<u8> = word.bytes().collect();
                for i in 0..wb.len() {
                    for ch in b'a'..=b'z' {
                        if wb[i] == ch {
                            continue;
                        }
                        let mut cand_bytes = wb.clone();
                        cand_bytes[i] = ch;
                        let cand = String::from_utf8(cand_bytes).unwrap();

                        if end_set.contains(&cand) {
                            return steps + 1;
                        }

                        if word_set.contains(&cand) {
                            next_level.insert(cand);
                        }
                    }
                }
            }
            start_set = next_level;
            steps += 1;
        }
        0
    }
}

fn main() {
    let w = |v: &[&str]| -> Vec<String> { v.iter().map(|s| s.to_string()).collect() };

    let wl1 = w(&["hot", "dot", "dog", "lot", "log", "cog"]);
    assert_eq!(
        Solution::ladder_length("hit".into(), "cog".into(), wl1.clone()),
        5
    );
    assert_eq!(
        Solution::ladder_length_bidirectional("hit".into(), "cog".into(), wl1),
        5
    );

    let wl2 = w(&["hot", "dot", "dog", "lot", "log"]);
    assert_eq!(
        Solution::ladder_length("hit".into(), "cog".into(), wl2.clone()),
        0
    );
    assert_eq!(
        Solution::ladder_length_bidirectional("hit".into(), "cog".into(), wl2),
        0
    );

    println!("All test cases passed for Word Ladder I (Standard BFS, Bidirectional BFS)!");
}
