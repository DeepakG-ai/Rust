// ============================================================================
// ## Q15 — lifetimes
//
// Write:
// - `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` — the longer string
// - a struct that holds a borrowed string:
// ```rust
// struct Highlight<'a> {
//     text: &'a str,
// }
// ```
// with a method `fn first_sentence(&self) -> &str`.
//
// In `main`, build a `String`, create a `Highlight` borrowing it, and print.
//
// Then break it on purpose: make the `String` go out of scope while the
// `Highlight` is still alive. Read the error, write it in a comment.
//
// Hint: `'a` does not change behaviour. It only tells the compiler "the output
// borrows from the input, so the input must outlive it".
// ============================================================================

struct Highlight<'a> {
    text: &'a str,
}

impl <'a> Highlight<'a> {
    fn first_sentence(&self)->&str{
        self.text.split('.').next().unwrap_or(self.text)
   
    }
}


fn longest<'a> (a:&'a str, b:&'a str)->&'a str{
    if a.len() > b.len(){
        return a
    }
    return b
}

fn main() {
    let s1 =String::from("Deepak");
    let s2 =String::from("Gowda");
    let ans = longest(&s1,&s2);
    println!("{}",ans);
    //{
    let text = String::from("Hello world. This is Rust.");//} if i do like this, text will go out of scope and raise error cannot find value text in this scope
    let h = Highlight { text: &text };
    println!("{}", h.first_sentence());
}
