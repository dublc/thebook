use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing
    let s = 5;
    let s = s + 1;
    {
        let s = s * 2;
        println!("The value of s in the inner scope is: {s}");
    }
    println!("The value of s is: {s}");

    // data type, rust is statically typed
    // 
    // scalar: intergers, floating-point, numbers, Booleans, characters(4 bytes)
    // compound: tuple, array
   

    // tuple, element in tuple can have different type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring
    let (_x, y, _z) = tup;
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;
    println!("The value of y is: {y}");

    // array, every element of array must have the same type
    // memory allocated on the stack
    let _a = [1,2,3,4,5];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // same as `let a = [3, 3, 3, 3, 3]`
    let a = [3; 5];
    let first = a[0];
    let _second = a[1];
    println!("the first element of array a is {first}");

    loop {
        println!("Please enter an array index");
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
        
        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        // panic when index out of bounds
        let element = a[index];
        println!("The value of element at index {index} is: {element}");

    }
}
