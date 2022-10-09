use rand::Rng;
use std::collections::HashMap;

fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    let mut s = String::new();
    let data = "initial contents";
    if data != "" {
        s = data.to_string();
    }
    println!("=================================================================");
    println!("{s}");

    // ==========================================================================
    let mut scores = HashMap::new();
    let field_name = String::from("Blue");

    scores.insert(&field_name, 10);
    scores.entry(&field_name).or_insert(50);

    let score = scores.get(&field_name);
    println!("{:#?}", score);

    median_mode();
}

fn median_mode() {
    println!("======================  call median_mode ========================");
    let mut nums = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 1..200 {
        let x = rng.gen_range(0..100);
        nums.push(x)
    }
    nums.sort();
    println!("{:?}", nums);
    let middle = nums.len() / 2;
    println!("==================================================================================");
    println!("median is {}", nums[middle]);

    let mut counts = HashMap::new();

    let mut mode = 0;
    let mut max_count = 0;
    for v in nums {
        let count = counts.entry(v).or_insert(0);
        *count += 1;
        if count > &mut max_count {
            max_count = *count;
            mode = v;
        }
    }
    println!("mode is {}", mode);
}
