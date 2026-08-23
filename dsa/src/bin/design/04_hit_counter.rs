// LeetCode Problem 362: Design Hit Counter
// Approaches:
//   1) Queue-Based: -> O(1) hit | O(N) getHits | O(N) space
//   2) Optimal (Fixed 300-Bucket Circular Array): -> O(1) hit | O(1) getHits | O(1) fixed space
// Link: https://leetcode.com/problems/design-hit-counter/
//
// Description:
//   Design a hit counter which counts the number of hits received in the past 5 minutes (300 seconds).

use std::collections::VecDeque;

/// 1. QUEUE-BASED HIT COUNTER:
#[derive(Default)]
pub struct HitCounterQueue {
    hits: VecDeque<i32>,
}

impl HitCounterQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hit(&mut self, timestamp: i32) {
        self.hits.push_back(timestamp);
    }

    pub fn get_hits(&mut self, timestamp: i32) -> i32 {
        while let Some(&front) = self.hits.front() {
            if timestamp - front >= 300 {
                self.hits.pop_front();
            } else {
                break;
            }
        }
        self.hits.len() as i32
    }
}

/// 2. OPTIMAL (Fixed 300-Bucket Circular Array):
/// Scalable for millions of concurrent requests occurring at the same timestamp.
pub struct HitCounter {
    times: [i32; 300],
    hits: [i32; 300],
}

impl Default for HitCounter {
    fn default() -> Self {
        Self {
            times: [0; 300],
            hits: [0; 300],
        }
    }
}

impl HitCounter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a hit at timestamp.
    /// Time: O(1) | Space: O(1)
    pub fn hit(&mut self, timestamp: i32) {
        let idx = (timestamp % 300) as usize;
        if self.times[idx] != timestamp {
            self.times[idx] = timestamp;
            self.hits[idx] = 1;
        } else {
            self.hits[idx] += 1;
        }
    }

    /// Return total hits in past 300 seconds [timestamp - 299 ..= timestamp].
    /// Time: O(300) = O(1) | Space: O(1)
    pub fn get_hits(&self, timestamp: i32) -> i32 {
        let mut total = 0;
        for i in 0..300 {
            if timestamp - self.times[i] < 300 {
                total += self.hits[i];
            }
        }
        total
    }
}

fn main() {
    let mut hc_q = HitCounterQueue::new();
    let mut hc = HitCounter::new();

    // hit at timestamp 1, 2, 3
    hc_q.hit(1);
    hc_q.hit(2);
    hc_q.hit(3);
    hc.hit(1);
    hc.hit(2);
    hc.hit(3);

    // get hits at timestamp 4 -> 3
    assert_eq!(hc_q.get_hits(4), 3);
    assert_eq!(hc.get_hits(4), 3);

    // hit at timestamp 300
    hc_q.hit(300);
    hc.hit(300);

    // get hits at timestamp 300 -> 4
    assert_eq!(hc_q.get_hits(300), 4);
    assert_eq!(hc.get_hits(300), 4);

    // get hits at timestamp 301 (timestamp 1 expires) -> 3
    assert_eq!(hc_q.get_hits(301), 3);
    assert_eq!(hc.get_hits(301), 3);

    println!("All test cases passed for Design Hit Counter (Queue, Fixed 300-Bucket Array O(1))!");
}
