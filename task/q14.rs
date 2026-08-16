struct Pair<T>{
    first:T,
    second:T,
}

impl <T>Pair<T>{
    fn new(first:T,second:T)->Pair<T>{
        Pair {
            first,
            second,
        }
    }

    fn swap(&mut self){
        std::mem::swap(&mut self.first, &mut self.second);
    }
}
impl <T: PartialOrd> Pair<T>{
    fn largest(&self)->&T{
        if self.first>=self.second{
            return &self.first;
        } else {
            &self.second
        }
    }

}

fn main() {
    // --- 1. Test with i32 ---
    let mut numbers = Pair::new(10, 25);
    println!("Initial: first = {}, second = {}", numbers.first, numbers.second);
    println!("Largest: {}", numbers.largest());

    numbers.swap();
    println!("After swap: first = {}, second = {}", numbers.first, numbers.second);

    // --- 2. Test with String ---
    let mut words = Pair::new(String::from("zebra"), String::from("apple"));
    println!("\nWords: first = {}, second = {}", words.first, words.second);
    println!("Largest word: {}", words.largest());

    words.swap();
    println!("After swap: first = {}, second = {}", words.first, words.second);
}


