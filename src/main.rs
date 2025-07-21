struct Rectangle {
    width: u32,
    height: u32,
}

// This function does not want to take ownership
// of the parameter, so it only borrows a
// reference. The main function can then use
// that Rectangle struct again after this function
// is called.
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
