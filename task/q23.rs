use std::time::Instant;

//tokio::spawn  →  creates a lightweight Tokio Task (green thread)
//std::thread::spawn  →  creates a heavy OS Thread

async fn fetch_user_data(user_id: u32, delay_ms: u64) -> String {
    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;// task has to will wait first as delay ms given, then load
    format!("User {user_id} data loaded")
}

#[tokio::main]
async fn main() {
    let start = Instant::now(); // this is for time start. 

    // Spawn 3 concurrent tasks
    let handle1 = tokio::spawn(fetch_user_data(1, 300));
    let handle2 = tokio::spawn(fetch_user_data(2, 100));
    let handle3 = tokio::spawn(fetch_user_data(3, 200));

    // Await all three
    let result1 = handle1.await.unwrap(); //join handle
    let result2 = handle2.await.unwrap();
    let result3 = handle3.await.unwrap();

    let elapsed = start.elapsed(); // end time

    //let tasks = vec![(1, 300), (2, 100), (3, 200)];
    //let mut handles = vec![];

    //for (id, delay) in tasks {
    //  let handle = tokio::spawn(fetch_user_data(id, delay));
    //  handles.push(handle);
//}

// Await all results
//  for handle in handles {
    //let result = handle.await.unwrap();
    //println!("{result}");
//}


    println!("{result1}");
    println!("{result2}");
    println!("{result3}");
    println!("Total elapsed: {elapsed:.2?}");
}
