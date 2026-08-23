// LeetCode Problem 297: Serialize and Deserialize Binary Tree
// Method: BFS level-order with "null" markers / DFS preorder
// Time: O(n) | Space: O(n)
//
// We MUST store nulls, otherwise the tree shape can't be recovered.
//
// Tree:      1
//           / \
//          2   3
//             / \
//            4   5
// BFS string: "1,2,3,null,null,4,5"
// DFS string: "1,2,null,null,3,4,null,null,5,null,null"

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

fn new_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode { val, left: None, right: None }))
}

/// Pretty-print tree sideways (helper like Python's print_tree)
fn print_tree(node: &Link, level: usize, prefix: &str) {
    if let Some(n) = node {
        println!("{}{}{}", " ".repeat(level * 4), prefix, n.borrow().val);
        print_tree(&n.borrow().left, level + 1, "L--- ");
        print_tree(&n.borrow().right, level + 1, "R--- ");
    }
}

// ---------- BFS Codec (level order) ----------
struct CodecBfs;

impl CodecBfs {
    fn serialize(root: &Link) -> String {
        if root.is_none() {
            return String::new();
        }
        let mut parts: Vec<String> = Vec::new();
        let mut queue = VecDeque::from([root.clone()]);

        while let Some(cur) = queue.pop_front() {
            match cur {
                Some(node) => {
                    parts.push(node.borrow().val.to_string());
                    queue.push_back(node.borrow().left.clone());
                    queue.push_back(node.borrow().right.clone());
                }
                None => parts.push("null".to_string()), // missing child marker
            }
        }

        // trim trailing nulls for a cleaner string
        while parts.last().map(|p| p == "null").unwrap_or(false) {
            parts.pop();
        }
        parts.join(",")
    }

    fn deserialize(data: &str) -> Link {
        if data.is_empty() {
            return None;
        }
        let vals: Vec<&str> = data.split(',').collect();
        let root = Some(new_node(vals[0].parse().unwrap()));
        let mut queue = VecDeque::from([root.clone()]);
        let mut i = 1;

        while !queue.is_empty() && i < vals.len() {
            let parent = queue.pop_front().unwrap().unwrap(); // Rc<RefCell<TreeNode>>
            // left child slot
            if vals[i] != "null" {
                let c = new_node(vals[i].parse().unwrap());
                parent.borrow_mut().left = Some(c.clone());
                queue.push_back(Some(c));
            }
            i += 1;
            // right child slot
            if i < vals.len() && vals[i] != "null" {
                let c = new_node(vals[i].parse().unwrap());
                parent.borrow_mut().right = Some(c.clone());
                queue.push_back(Some(c));
            }
            i += 1;
        }
        root
    }
}

// ---------- DFS Codec (preorder) ----------
struct CodecDfs;

impl CodecDfs {
    fn serialize(root: &Link) -> String {
        fn dfs(node: &Link, out: &mut Vec<String>) {
            match node {
                None => out.push("null".into()),
                Some(n) => {
                    out.push(n.borrow().val.to_string()); // Root
                    dfs(&n.borrow().left, out); // Left
                    dfs(&n.borrow().right, out); // Right
                }
            }
        }
        let mut out = Vec::new();
        dfs(root, &mut out);
        out.join(",")
    }

    fn deserialize(data: &str) -> Link {
        if data.is_empty() {
            return None;
        }
        let vals: Vec<Option<i32>> = data
            .split(',')
            .map(|v| if v == "null" { None } else { v.parse::<i32>().ok() })
            .collect();

        fn dfs(vals: &[Option<i32>], i: &mut usize) -> Link {
            if *i >= vals.len() || vals[*i].is_none() {
                *i += 1;
                return None;
            }
            let node = new_node(vals[*i].unwrap());
            *i += 1;
            node.borrow_mut().left = dfs(vals, i);
            node.borrow_mut().right = dfs(vals, i);
            Some(node)
        }

        let mut i = 0;
        dfs(&vals, &mut i)
    }
}

fn main() {
    // Build:   1 -> 2, 3 ; 3 -> 4, 5
    let root = Some(new_node(1));
    root.as_ref().unwrap().borrow_mut().left = Some(new_node(2));
    let r3 = new_node(3);
    r3.borrow_mut().left = Some(new_node(4));
    r3.borrow_mut().right = Some(new_node(5));
    root.as_ref().unwrap().borrow_mut().right = Some(r3);

    // BFS round trip
    let s_bfs = CodecBfs::serialize(&root);
    assert_eq!(s_bfs, "1,2,3,null,null,4,5");
    let back = CodecBfs::deserialize(&s_bfs);
    assert_eq!(CodecBfs::serialize(&back), s_bfs);

    // DFS round trip
    let s_dfs = CodecDfs::serialize(&root);
    assert_eq!(s_dfs, "1,2,null,null,3,4,null,null,5,null,null");
    let back2 = CodecDfs::deserialize(&s_dfs);
    assert_eq!(CodecDfs::serialize(&back2), s_dfs);

    // empty tree round trips to empty
    assert_eq!(CodecBfs::serialize(&None), "");
    assert!(CodecBfs::deserialize("").is_none());

    print_tree(&root, 0, "Root: ");
    println!("All test cases passed!");
}
