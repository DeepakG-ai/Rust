// ============================================================================
// ## Q3 — borrow vs move (the important one)
//
// Write two functions that both add up a list of prices:
// - `fn total_borrowed(prices: &Vec<f64>) -> f64`
// - `fn total_owned(prices: Vec<f64>) -> f64`
//
// In `main`:
// 1. build `let prices = vec![10.5, 20.0, 3.25];`
// 2. call `total_borrowed(&prices)` — then print `prices` again. Works.
// 3. call `total_owned(prices)` — then try to print `prices` again.
// 4. The compiler will refuse. Read the error message fully, then comment
//    that line out and write the error as a comment in your file.
// ============================================================================

fn main (){
    let prices = vec![10.5,20.0, 3.25];
    let ans1 = total_borrowed(&prices);
    println!("borrowed:{}",ans1);
    println!("borrowed prices:{:?}",prices);

    let ans = total_owned(prices);
    println!("owned:{}",ans);
    //println!("owned prices:{:?}",prices);
// ERROR: borrow of moved value: `prices`
// value moved into total_owned(), can't use it after
}

// BORROW: "hey function, LOOK at my prices, then give back"
fn total_borrowed(prices:& Vec<f64>)->f64{
    let mut total:f64 = 0.0;
    for i in prices{
        total = total +i
    }
    return total;
}
// MOVE: "hey function, TAKE my prices, they're yours now" 
fn total_owned(prices:Vec<f64>)->f64{
    let mut total=0.0;
    for i in prices{
        total = total+i
    }
    return total;
}

//borrowed values will come back to variable again. but moving values to function will not come back. it belongs to function.. not variables..