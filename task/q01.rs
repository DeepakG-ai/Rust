// ============================================================================
// ## Q1 — for loop and if
//
// Write a program that:
// - loops over the numbers 1 to 20
// - skips any number divisible by 3
// - prints every other number
// - after the loop, prints the total of the numbers it printed
//
// Expected output ends with:
// 1
// 2
// 4
// ...
// 20
// Total: 147
//
// Hint: `for n in 1..=20 { }`, and `continue` skips to the next iteration.
// ============================================================================

fn main (){
    let total: i32= 0; // let mut total:i32=0 this give warning.
    let result = q1_sum(total);
    println!("Total : {}",result);
}

fn q1_sum(mut total:i32)->i32{ // we need mention mut here, it creates own copy of total variable. 
    for i in 1..20{
        if i%3==0 {
            continue;
        }
        println!("{}",i);

        total = total +i;
    }
    return total;
}
// Why?
//Because mut in the caller (main) only means "I can change MY copy". It doesn't make the function's copy mutable. They are separate variables.