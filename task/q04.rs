//Parameters: use &str (90% of the time) Return new string: use String
//&str, will at stack. not heap because it memory is fixed. we can't use text.push_str() like that. Sting will on heap. so it will dynamicaly change

fn main(){
    let mut text = String::from("hello world");

    println!("{}",shout(&text));
    println!("{}",first_word(&text));
    add_prefix(&mut text, "[LOG]"); // this function is not return text, it just return nothing. so that we can't use it like println!(add_prefix(&mut text, "[LOG]"));
    println!("{} ",text);

}
fn shout(text:&str)->String{
    let mut result = text.to_uppercase();
    result.push('!'); //new string output, so String
    result
}

fn first_word(text:&str)->&str{ //first_word(text: &str) -> &str — reads input, returns a piece of it
    text.split_whitespace().next().unwrap()
    //let t = text.split_whitespace().next().unwrap(); if return text, it will not work, becuase not storing or saving anywhere. 
    //return t;
   
}
fn add_prefix(text:&mut String, prefix: &str){ //add_prefix(text: &mut String, prefix: &str) — modifies the String, so needs &mut String
    text.insert_str(0, prefix);
}
// if we return like that