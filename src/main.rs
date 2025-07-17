struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect = Rectangle {
        width: 70,
        height: 70,
    };

    println!(
        "The area of the rectangle is {} unit squared.",
        area(&rect)
    );
}
