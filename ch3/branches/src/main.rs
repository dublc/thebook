use std::io;

fn main() {
    let mut number = String::new();

    println!("Please enter a number");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read from stdin");

    let number: i32 = number.trim().parse().expect("Can not parse to i32 type");

    //condition must be a bool
    if number < 10 {
        // blocks of code associated with the condition in 
        // if expressions are sometimes called arms
        println!("condition was smaller than 10");
    } else if number < 20 {
        println!("condition was smaller than 20");
    } else {
        println!("all the other condition")
    }

    // use if in let statement right side
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("num is: {num}");

    let return_value = repeat_with_for();
    println!("return value is: {return_value}");


}

#[allow(dead_code)]
fn repeat_with_loop() -> i32 {
    let mut count = 0;
    'count: loop {
        println!("count = {count}");

        let mut inner_count = 10;
        loop {
            println!("inner_count = {inner_count}");
            if inner_count == 9 {
                break;
            }
            if count == 2 {
                break 'count;
            }
            inner_count -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    count
}

#[allow(dead_code)]
fn repeat_with_while() -> i32 {
    let mut count = 0;
    while count < 2 {
        let mut inner_count = 10;
        while inner_count > 5 {
            println!("inner count value is {inner_count}");
            inner_count -= 1;
        }
        count += 1
    }
    count
}


fn repeat_with_for() -> i32 {
    for number in 0..2 {
        println!("number is {number}");
    }
    2
}
