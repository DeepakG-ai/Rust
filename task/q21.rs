// ============================================================================
// ## Q21 — Valid Anagram (String)
//
// ```rust
// fn is_anagram(s: String, t: String) -> bool
// ```
//
// Examples:
// - "anagram", "nagaram"  ->  true
// - "rat", "car"          ->  false
// - "ab", "a"             ->  false
//
// Hint: different lengths → immediately false. Then count characters of s in
// a HashMap<char, i32>, subtract the counts of t, and check nothing is left
// over. `s.chars()` gives you characters.
// ============================================================================

use std::collections::HashMap;

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut counts: HashMap<char, i32> = HashMap::new();

    // Count frequency of each character in s
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    // Decrement frequency for each character in t
    for c in t.chars() {
        let count = counts.entry(c).or_insert(0);
        *count -= 1;
        if *count < 0 {
            return false;
        }
    }

    // Ensure all frequencies are 0
    counts.values().all(|&val| val == 0)
}

fn main() {
    // Test 1: "anagram", "nagaram" -> true
    let result1 = is_anagram(String::from("anagram"), String::from("nagaram"));
    println!("\"anagram\", \"nagaram\" -> {}", result1);

    // Test 2: "rat", "car" -> false
    let result2 = is_anagram(String::from("rat"), String::from("car"));
    println!("\"rat\", \"car\" -> {}", result2);

    // Test 3: "ab", "a" -> false
    let result3 = is_anagram(String::from("ab"), String::from("a"));
    println!("\"ab\", \"a\" -> {}", result3);
}
