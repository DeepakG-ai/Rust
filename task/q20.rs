use std::collections::HashMap;

fn two_sum(nums:Vec<i32>,target:i32)->Vec<i32>{
    let mut map = HashMap::new();
    for (i, num) in nums.into_iter().enumerate() {
        let diff = target -num;
        if let Some(&prev_index) = map.get(&diff) {
            return vec![prev_index as i32, i as i32];
        }
        map.insert(num, i);
    
    }
    vec![]

}

//fn two_sum(nums:Vec<i32>,target:i32)->Vec<i32>{
//    for i in 0..nums.len(){
//        for j in (i+1)..nums.len(){
//            let sum:i32;
//           sum = nums[i]+nums[j];
//            if target == sum{
//                return vec![i as i32,j as i32];
//            }
//        }
//    }
//    return vec![];
//}

fn main() {
    // Test 1: [2, 7, 11, 15], target = 9 -> [0, 1]
    let ans1 = two_sum(vec![2, 7, 11, 15], 9);
    println!("Test 1: {:?}", ans1);

    // Test 2: [3, 2, 4], target = 6 -> [1, 2]
    let ans2 = two_sum(vec![3, 2, 4], 6);
    println!("Test 2: {:?}", ans2);

    // Test 3: [3, 3], target = 6 -> [0, 1]
    let ans3 = two_sum(vec![3, 3], 6);
    println!("Test 3: {:?}", ans3);
}
