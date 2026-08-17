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
