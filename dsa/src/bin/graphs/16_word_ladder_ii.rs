// Word Ladder II - LeetCode 126 (HARD)
// Method: BFS carrying full PATHS; remove used words only after the level
// Time: exponential in worst case (all shortest paths requested!)
//
// Difference from Word Ladder I:
//   I  -> return LENGTH of one shortest path
//   II -> return ALL shortest paths
//
// KEY TRICK: words used at the CURRENT level stay valid for siblings at the
// same level (multiple paths may pass through the same word), so we collect
// `used_this_level` and purge it only when the level finishes.
//
// Example:
//   begin="hit", end="cog", list=[hot,dot,dog,lot,log,cog]
//   -> [["hit","hot","dot","dog","cog"],
//       ["hit","hot","lot","log","cog"]]

use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut word_set: HashSet<String> = word_list.into_iter().collect();
        let mut result: Vec<Vec<String>> = Vec::new();

        if !word_set.contains(&end_word) {
            return result;
        }
        word_set.remove(&begin_word);

        // BFS queue stores the whole path reaching each word
        let mut q: VecDeque<(String, Vec<String>)> =
            VecDeque::from([(begin_word.clone(), vec![begin_word])]);

        let mut found = false;

        while !q.is_empty() && !found {
            let level_size = q.len();
            let mut used_this_level: HashSet<String> = HashSet::new();

            for _ in 0..level_size {
                let (word, path) = q.pop_front().unwrap();

                // generate all one-letter substitutions
                let wb: Vec<u8> = word.bytes().collect();
                for i in 0..wb.len() {
                    for ch in b'a'..=b'z' {
                        if wb[i] == ch {
                            continue;
                        }
                        let mut cand_bytes = wb.clone();
                        cand_bytes[i] = ch;
                        let cand = String::from_utf8(cand_bytes).unwrap();

                        if word_set.contains(&cand) {
                            let mut new_path = path.clone();
                            new_path.push(cand.clone());

                            if cand == end_word {
                                result.push(new_path); // record this complete path
                                found = true; // keep draining THIS level only
                            } else if !found {
                                used_this_level.insert(cand.clone());
                                q.push_back((cand, new_path));
                            }
                        }
                    }
                }
            }

            // purge AFTER the whole level: siblings may still need these words
            for w in used_this_level {
                word_set.remove(&w);
            }
        }
        result
    }
}

fn main() {
    let w = |v: &[&str]| -> Vec<String> { v.iter().map(|s| s.to_string()).collect() };

    let paths = Solution::find_ladders(
        "hit".into(),
        "cog".into(),
        w(&["hot", "dot", "dog", "lot", "log", "cog"]),
    );
    assert_eq!(paths.len(), 2);
    assert!(paths.contains(&w(&["hit", "hot", "dot", "dog", "cog"])));
    assert!(paths.contains(&w(&["hit", "hot", "lot", "log", "cog"])));

    // no transformation possible
    assert_eq!(
        Solution::find_ladders("hit".into(), "cog".into(), w(&["hot", "dot", "dog", "lot", "log"])),
        Vec::<Vec<String>>::new()
    );

    println!("All test cases passed!");
}
