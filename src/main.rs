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
    // The &self parameter is shorthand for self: &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.width > rect.height && self.height > rect.width && self.height > rect.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 70,
        height: 70,
    };

    println!("The area of the rect1 is {} unit squared (function).", area(&rect1));
    println!("The area of the rect1 is {} unit squared (method).", rect1.area());

    // Without specifying :? or :#? in the curly brackets, 
    // the Display format will be used, which is not
    // implemented by a struct. So :? or :#? is used to
    // specify the use of Debug format. :#? can make the
    // expression of the struct prettier.
    println!("rect1 is a struct: {rect1:?}.");
    println!("rect1 is a struct: {rect1:#?}.");

    let rect2 = Rectangle {
        width: 50,
        height: 60,
    };

    let rect3 = Rectangle {
        width: 100,
        height: 70,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3))
}
