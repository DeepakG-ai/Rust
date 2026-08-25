fn sink(grid: &mut Vec<Vec<char>>, r: usize, c: usize) {
    let rows = grid.len();
    let cols = grid[0].len();

    // If current cell is water or already visited, return
    if grid[r][c] != '1' {
        return;
    }

    // "Sink" the current land cell
    grid[r][c] = '0';

    // Recursively sink 4-directionally adjacent land (up, down, left, right)
    if r > 0 {
        sink(grid, r - 1, c); // up
    }
    if r + 1 < rows {
        sink(grid, r + 1, c); // down
    }
    if c > 0 {
        sink(grid, r, c - 1); // left
    }
    if c + 1 < cols {
        sink(grid, r, c + 1); // right
    }
}

fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' {
                count += 1;
                // Sink the entire connected island
                sink(&mut grid, r, c);
            }
        }
    }

    count
}

fn main() {
    // Test 1: Grid from task.md (3 islands)
    let grid1 = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    let ans1 = num_islands(grid1);
    println!("Test 1 Islands: {} (expected 3)", ans1);

    // Test 2: Single big island
    let grid2 = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    let ans2 = num_islands(grid2);
    println!("Test 2 Islands: {} (expected 1)", ans2);

    // Test 3: No land (0 islands)
    let grid3 = vec![
        vec!['0', '0', '0'],
        vec!['0', '0', '0'],
    ];
    let ans3 = num_islands(grid3);
    println!("Test 3 Islands: {} (expected 0)", ans3);
}
