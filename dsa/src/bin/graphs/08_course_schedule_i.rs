// Course Schedule I (Can Finish?) - LeetCode 207
// Method: Kahn's algorithm = cycle detection with inverted answer
// Time: O(V + E) | Space: O(V + E)
//
// [course, prereq] pairs -> edge prereq -> course.
// Can finish all courses  <=>  the dependency graph has NO cycle.
// Kahn's processes exactly V nodes when acyclic.
//
// Examples:
//   numCourses=2, [[1,0]]        -> true   (take 0 then 1)
//   numCourses=2, [[1,0],[0,1]]  -> false  (deadlock!)

use std::collections::VecDeque;

pub fn can_finish(num_courses: usize, prerequisites: Vec<Vec<i32>>) -> bool {
    // build adjacency: prereq -> course
    let mut adj = vec![Vec::new(); num_courses];
    let mut in_degree = vec![0usize; num_courses];
    for p in &prerequisites {
        let course = p[0] as usize;
        let prereq = p[1] as usize;
        adj[prereq].push(course);
        in_degree[course] += 1;
    }

    // start with all courses that have no prerequisites
    let mut q: VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .filter(|(_, &d)| d == 0)
        .map(|(i, _)| i)
        .collect();

    let mut taken = 0usize;
    while let Some(course) = q.pop_front() {
        taken += 1; // take this course!
        for &next in &adj[course] {
            in_degree[next] -= 1; // one prerequisite satisfied
            if in_degree[next] == 0 {
                q.push_back(next); // unlocked!
            }
        }
    }

    taken == num_courses // took everything => no cycle => true
}

fn main() {
    // TEST 1: simple valid order
    assert!(can_finish(2, vec![vec![1, 0]]));

    // TEST 2: circular dependency
    assert!(!can_finish(2, vec![vec![1, 0], vec![0, 1]]));

    // TEST 3: diamond dependencies, still acyclic
    assert!(can_finish(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]));

    // TEST 4: no prerequisites at all
    assert!(can_finish(3, vec![]));

    println!("All test cases passed!");
}
