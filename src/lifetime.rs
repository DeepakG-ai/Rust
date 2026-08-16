fn main() {
    let s1 = String::from("Deepak");
    let s2 = String::from("Gowda");
    let ans = longest(&s1, &s2);
    println!("{}", ans)
}

fn longest<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        return s1;
    } else {
        return s2;
    }
}

//this function we can't write, without lifetimes.