// Network Delay Time - LeetCode 743
// Approaches:
//   1) Better (Bellman-Ford Algorithm): Relax all edges (V - 1) times -> O(V * E) time | O(V) space
//   2) Optimal (Dijkstra's Algorithm with Min-Heap): -> O((V + E) log V) time | O(V + E) space
// Link: https://leetcode.com/problems/network-delay-time/
//
// Examples:
//   times=[[2,1,1],[2,3,1],[3,4,1]], n=4, k=2 -> 2

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    /// 1. BELLMAN-FORD ALGORITHM:
    /// Relax every edge (n - 1) times.
    /// Time: O(n * E) | Space: O(n)
    pub fn network_delay_time_bellman_ford(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let src = k as usize;
        let mut dist = vec![i64::MAX; n + 1];
        dist[src] = 0;

        for _ in 1..n {
            let mut updated = false;
            for t in &times {
                let (u, v, w) = (t[0] as usize, t[1] as usize, t[2] as i64);
                if dist[u] != i64::MAX && dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                    updated = true;
                }
            }
            if !updated {
                break;
            }
        }

        let max_dist = dist[1..=n].iter().max().copied().unwrap_or(i64::MAX);
        if max_dist == i64::MAX { -1 } else { max_dist as i32 }
    }

    /// 2. OPTIMAL (Dijkstra's Algorithm with Min-Heap):
    /// Greedy shortest path from source k.
    /// Time: O((V + E) log V) | Space: O(V + E)
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let src = k as usize;

        let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n + 1];
        for t in &times {
            let (u, v, w) = (t[0] as usize, t[1] as usize, t[2] as i64);
            adj[u].push((v, w));
        }

        let mut dist = vec![i64::MAX; n + 1];
        dist[src] = 0;
        let mut pq: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        pq.push(Reverse((0, src)));

        while let Some(Reverse((d, node))) = pq.pop() {
            if d > dist[node] {
                continue;
            }
            for &(nb, w) in &adj[node] {
                if d + w < dist[nb] {
                    dist[nb] = d + w;
                    pq.push(Reverse((dist[nb], nb)));
                }
            }
        }

        let max_dist = dist[1..=n].iter().max().copied().unwrap_or(i64::MAX);
        if max_dist == i64::MAX { -1 } else { max_dist as i32 }
    }
}

fn main() {
    let test_cases = vec![
        (vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2, 2),
        (vec![], 1, 1, 0),
        (
            vec![
                vec![1, 2, 1],
                vec![2, 3, 7],
                vec![1, 3, 9],
                vec![3, 4, 10],
            ],
            4,
            1,
            18,
        ),
        (vec![vec![1, 2, 1]], 2, 2, -1),
    ];

    for (times, n, k, expected) in test_cases {
        assert_eq!(
            Solution::network_delay_time_bellman_ford(times.clone(), n, k),
            expected
        );
        assert_eq!(
            Solution::network_delay_time(times, n, k),
            expected
        );
    }

    println!("All test cases passed for Network Delay Time (Bellman-Ford, Dijkstra)!");
}
