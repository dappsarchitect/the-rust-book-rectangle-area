// To explicitly specify the Debug output format (trait) for
// the struct with the outer attribute #[derive(Debug)]
#[derive(Debug)]
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

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 70,
        height: 70,
    };

    println!("The area of the rectangle is {} unit squared (function).", area(&rect));
    println!("The area of the rectangle is {} unit squared (method).", rect.area());

    // Without specifying :? in the curly brackets, 
    // the Display format will be used, which is not
    // implemented by a struct. So :? is used to
    // specify the use of Debug format. :#? can also
    // be used so the struct fields are listed clearly.
    println!("The rectangle is a struct: {rect:?}.");
}
