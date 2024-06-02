use std::io;

struct Point {
    x: i32,
    y: i32
}
struct Rectangle {
    upper_left: Point,
    lower_right: Point
}

fn area_of_rectangle(rect: &Rectangle) -> i32 {
    let width: i32 = rect.lower_right.x - rect.upper_left.x;
    let height: i32 = rect.upper_left.y - rect.lower_right.y;
    return width * height;
}

fn center_of_rectangle(rect: &Rectangle) -> Point {
    let p: Point = Point { 
        x: (rect.upper_left.x + rect.lower_right.x) / 2, 
        y: (rect.upper_left.y + rect.lower_right.y) / 2
    };
    return p;
}

fn main() {
    println!("Please enter upper_left and lower_right points ");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let inputs: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let r: Rectangle = Rectangle { 
        upper_left: Point { x: inputs[0], y: inputs[1] }, 
        lower_right: Point { x: inputs[2], y: inputs[3] } 
    };

    let area: i32 = area_of_rectangle(&r);
    println!("The area of the rectangle is: {}", area);

    let center: Point = center_of_rectangle(&r);
    println!("The center of the rectangle is: {}, {}", center.x, center.y);
}
