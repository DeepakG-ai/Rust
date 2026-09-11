fn main() {
    let name = String::from("Deepak");
    let mut count = 0;

    // 1. This closure only READS `name` → Rust assigns: Fn
    let read_only = || {
        println!("Hello {name}");  // just reading
    };

    // 2. This closure MUTATES `count` → Rust assigns: FnMut
    let mut mutator = || {
        count += 1;                // modifying captured variable
        println!("count = {count}");
    };

    // 3. This closure MOVES `name` out → Rust assigns: FnOnce
    let consumer = || {
        let moved = name;          // takes ownership, `name` is gone after this
        println!("consumed: {moved}");
    };

    // You can call read_only and mutator multiple times:
    read_only();
    read_only();
    mutator();
    mutator();

    // But consumer can only be called ONCE:
    consumer();
    // consumer(); // ❌ ERROR: value used after move
}
