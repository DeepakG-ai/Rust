// Course Schedule II (Order to Take Courses) - LeetCode 210
// Method: Topological sort via Kahn's algorithm
// Time: O(V + E) | Space: O(V + E)
//
// Same as Course Schedule I but RETURN the order instead of true/false.
// If a cycle exists (can't take everything), return [].
//
// Examples:
//   numCourses=4, [[1,0],[2,0],[3,1],[3,2]] -> [0,1,2,3]
//   numCourses=2, [[1,0],[0,1]]             -> []

use std::collections::VecDeque;

pub fn find_order(num_courses: usize, prerequisites: Vec<Vec<i32>>) -> Vec<usize> {
    // adjacency: prereq -> course ; count incoming edges
    let mut adj = vec![Vec::new(); num_courses];
    let mut in_degree = vec![0usize; num_courses];
    for p in &prerequisites {
        let (course, prereq) = (p[0] as usize, p[1] as usize);
        adj[prereq].push(course);
        in_degree[course] += 1;
    }

    let mut q: VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .filter(|(_, &d)| d == 0)
        .map(|(i, _)| i)
        .collect();

    let mut order = Vec::with_capacity(num_courses);

    while let Some(course) = q.pop_front() {
        order.push(course); // take this course now
        for &next in &adj[course] {
            in_degree[next] -= 1;
            if in_degree[next] == 0 {
                q.push_back(next);
            }
        }
    }

    if order.len() == num_courses { order } else { Vec::new() } // cycle -> []
}

fn main() {
    // TEST 1: classic example - multiple valid orders exist
    let o1 = find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]);
    assert!(o1 == vec![0, 1, 2, 3] || o1 == vec![0, 2, 1, 3]);

    // TEST 2: impossible (cycle)
    assert_eq!(find_order(2, vec![vec![1, 0], vec![0, 1]]), Vec::<usize>::new());

    // TEST 3: single course
    assert_eq!(find_order(1, vec![]), vec![0]);

    // TEST 4: chain
    assert_eq!(find_order(3, vec![vec![1, 0], vec![2, 1]]), vec![0, 1, 2]);

    println!("All test cases passed!");
}
