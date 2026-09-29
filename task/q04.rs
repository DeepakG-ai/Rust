// ============================================================================
// ## Q4 — `String` vs `&str`
//
// Write:
// - `fn shout(text: &str) -> String` — uppercase, with `"!"` on the end
// - `fn first_word(text: &str) -> &str` — the text up to the first space
// - `fn add_prefix(text: &mut String, prefix: &str)` — inserts at the front,
//   changing the caller's variable
//
// In `main`, prove all three work.
//
// Expected:
// HELLO WORLD!
// hello
// [LOG] hello world
//
// Hint: `split_whitespace().next()`, and `insert_str(0, ..)`.
// ============================================================================

//Parameters: use &str (90% of the time) Return new string: use String
//&str, will at stack. not heap because it memory is fixed. we can't use text.push_str() like that. Sting will on heap. so it will dynamicaly change

fn main(){
    let mut text = String::from("hello world");

    println!("{}",shout(&text));
    println!("{}",first_word(&text));
    add_prefix(&mut text, "[LOG]"); // this function is not return text, it just return nothing. so that we can't use it like println!(add_prefix(&mut text, "[LOG]"));
    println!("{} ",text);

}
fn shout(text: &str) -> String {
    // text.to_uppercase() allocates a brand-new `String` on the heap.
    // result is `String` (not &str), so it owns its buffer and CAN grow with `.push()`.
    // Note: `text` itself is untouched and never grows.
    let mut result = text.to_uppercase();
    result.push('!');
    result
}

fn first_word(text: &str) -> &str {
    // Both of these work identically:
    // Option 1 (direct):
    text.split_whitespace().next().unwrap()

    // Option 2 (with variable):
    // let t = text.split_whitespace().next().unwrap();
    // return t; // Works! `t` is a &str slice pointing to the first word ("hello").
    //
    // Note: `return text;` would NOT work to get the first word because `split_whitespace()`
    // does not mutate `text`. `text` still points to the full string ("hello world").
}

fn add_prefix(text: &mut String, prefix: &str) {
    // Must be `&mut String`, NOT `&mut str`:
    // - `String` owns a heap buffer with capacity and can grow when inserting characters.
    // - `str` has a fixed length with no allocator, so it cannot grow or shrink.
    // - That is why `insert_str` exists on `String`, but not on `str` or `&mut str`.
    text.insert_str(0, prefix);
}
