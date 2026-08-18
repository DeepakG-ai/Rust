fn main() {
    // ── Q1 rewrite: sum of 1..=20, skipping multiples of 3 ──

    // for-loop version:
    // let mut total = 0;
    // for n in 1..=20 {
    //     if n % 3 == 0 { continue; }
    //     total += n;
    // }

    let sum: i32 = (1..=20).filter(|n| n % 3 != 0).sum();
    println!("Q1 sum (skip multiples of 3): {}", sum);

    // ── Q3 rewrite: total of prices ──

    // for-loop version:
    // let mut total = 0.0;
    // for p in &prices {
    //     total += p;
    // }

    let prices = vec![10.5, 20.0, 3.25];
    let total: f64 = prices.iter().sum();
    println!("Q3 total of prices: {}", total);

    // ── New: numbers over 10 formatted as "n=15" ──

    // for-loop version:
    // let mut result = Vec::new();
    // for n in &numbers {
    //     if *n > 10 {
    //         result.push(format!("n={}", n));
    //     }
    // }

    let numbers = vec![15, 8, 42, 4, 23, 16];

    let over_ten: Vec<String> = numbers
        .iter()
        .filter(|&&n| n > 10)
        .map(|n| format!("n={}", n))
        .collect();

    println!("Over 10: {:?}", over_ten);

    // ── New: count how many are even ──

    // for-loop version:
    // let mut count = 0;
    // for n in &numbers {
    //     if n % 2 == 0 { count += 1; }
    // }

    let even_count = numbers.iter().filter(|&&n| n % 2 == 0).count();
    println!("Even count: {}", even_count);
}
