fn main() {
    let mut s = String::from("hello");
    s.push_str(". world!");

    println!("{s}");
    

    let s1 = String::from("hello");
    // s1 move to s2. s1 no longer valid
    let s2 = s1;
    println!("{s2}, world!");

    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}");


    takes_ownership(s3);    // s3 not avaliable from here
    // println!("{s3}");  error 

    let x = 5;
    makes_copy(x);

    // automatically call drop
}

fn takes_ownership(some_string: String) {
    println!("{some_string} in takes_ownership");
}

fn makes_copy(x: i32) {
    println!("{x} in makes_copy");
}
