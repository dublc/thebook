#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rectangle is {} square pixels.", rec.area());
    println!("=================================================================");

    if rec.width() {
        println!("The rectangle has a nonzero width; it is {}", rec.width);
    }
    println!("=================================================================");

    let rec2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("Can rec hold rect2? {}", rec.can_hold(&rec2));

    println!("=================================================================");
    dbg!(&rec);
}

//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}

#[allow(dead_code)]
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[allow(dead_code)]
fn area_v1(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}
