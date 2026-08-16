
fn largest<T:PartialOrd>(list:&[T])->&T{
    let mut biggest = &list[0];
    for i in list{
        if biggest<i{
            biggest= i;
        }

        }
        return biggest;
    }



fn main() {
    let numbers = [34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let floats = [1.2, 9.8, 3.3];
    println!("Largest float: {}", largest(&floats));

    let words = ["apple", "zebra", "mango"];
    println!("Largest word: {}", largest(&words));
    //"zebra" > "mango" > "apple" "apple" starts with 'a' (ASCII 97) "mango" starts with 'm' (ASCII 109),"zebra" starts with 'z' (ASCII 122)
}
