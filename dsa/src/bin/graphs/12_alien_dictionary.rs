// Alien Dictionary - LeetCode 269
// Method: build precedence graph from adjacent words + Kahn's topo sort
// Time: O(C) total characters | Space: O(1) (<= 26 nodes)
//
// For each adjacent pair of words, the FIRST differing character gives an
// edge c1 -> c2 ("c1 comes before c2"). Topologically sort all letters.
//
// INVALID cases:
//   - prefix violation: ["abc","ab"] -> "abc" can't come before its prefix
//   - cycle among letters -> result shorter than unique letter count
//
// Examples:
//   ["wrt","wrf","er","ett","rftt"] -> "wertf"
//   ["z","x"]                       -> "zx"
//   ["z","x","z"]                   -> ""   (cycle: z before x before z)

use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub fn alien_order(words: &[String]) -> String {
    // BTree keeps deterministic ordering; adj[c] = set of chars after c
    let mut adj: BTreeMap<u8, BTreeSet<u8>> = BTreeMap::new();
    let mut in_degree: BTreeMap<u8, usize> = BTreeMap::new();

    // every character that appears must be in the graph
    for w in words {
        for &b in w.as_bytes() {
            in_degree.entry(b).or_insert(0);
        }
    }

    // compare each ADJACENT pair of words
    for pair in words.windows(2) {
        let (w1, w2) = (pair[0].as_bytes(), pair[1].as_bytes());

        // prefix violation: longer word can't precede its own prefix
        if w1.len() > w2.len() && w1.starts_with(w2) {
            return String::new();
        }

        let len = w1.len().min(w2.len());
        for j in 0..len {
            if w1[j] != w2[j] {
                // edge: w1[j] -> w2[j]; set dedupes repeated edges
                if adj.entry(w1[j]).or_default().insert(w2[j]) {
                    *in_degree.get_mut(&w2[j]).unwrap() += 1;
                }
                break; // only FIRST difference matters
            }
        }
    }

    // Kahn's BFS over letters
    let mut q: VecDeque<u8> = in_degree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(&c, _)| c)
        .collect();

    let mut order = String::new();
    while let Some(c) = q.pop_front() {
        order.push(c as char);
        if let Some(nbs) = adj.get(&c) {
            for &nb in nbs {
                *in_degree.get_mut(&nb).unwrap() -= 1;
                if *in_degree.get(&nb).unwrap() == 0 {
                    q.push_back(nb);
                }
            }
        }
    }

    // not all letters placed => cycle => invalid
    if order.len() != in_degree.len() {
        String::new()
    } else {
        order
    }
}

fn main() {
    let w = |v: &[&str]| -> Vec<String> { v.iter().map(|s| s.to_string()).collect() };

    assert_eq!(alien_order(&w(&["wrt", "wrf", "er", "ett", "rftt"])), "wertf");
    assert_eq!(alien_order(&w(&["z", "x"])), "zx");
    assert_eq!(alien_order(&w(&["z", "x", "z"])), ""); // cycle
    assert_eq!(alien_order(&w(&["abc", "ab"])), ""); // prefix violation

    println!("All test cases passed!");
}
