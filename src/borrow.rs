fn main() {
    let mut name = String::from("Deepak");
    let name2 = &mut name;
    name2.push_str("\t Gowda");
    println!("name 2 {}", name2);

    let name3 = &name; //it will show error
    //let name4 = &name;

    println!("name 3 {}", name3); //if print name2 here, it will give error. it has out of scope.
    println!("name 1 {}", name)
}
