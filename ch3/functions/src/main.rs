fn main() {
    print_labeled_measurement(5, 'h');
    
    let x = five(5);
    println!("The value of x is: {x}");

}

#[allow(dead_code)]
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five(x: i32) -> i32 {
    // without semicolon, is a expression
    x + 1
}
