// define a struct named 'Shape'
// #[derive(Debug)]
// struct Shape {
//     width: u32,
//     height: u32,
// }

/*
RUST doesnt have a NULL value in the traditional concept
Instead, RUST implements the concept of 'Option<T>' ; you dont need to import it; its already there in the prelude.
enum Option<T> {
    Some(T), // not null, does have a value
    None, // null
 }
 */

// define an Enum named 'Shape' with possible values 'Circle', 'Square', 'Rectangle'
#[derive(Debug)]
enum Shape {
    Circle(f64), // (radius)
    Square(f64), // (side)
    Rectangle(f64, f64), // (width, height)
}

// create an implementation of Shape
impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Rectangle(width, height) => width * height,
        }
    }
    fn perimeter(&self) -> f64 {
        match *self { // see, we added explicit dereferencing *self, which is not required in RUST 1.70, as compiler will automatically dereferencing it
            Shape::Circle(radius) => 2.0 * std::f64::consts::PI * radius,
            Shape::Square(side) => 4.0 * side,
            Shape::Rectangle(width, height) => 2.0 * (width + height),
        }
    }
    fn display(&self) -> String {
        match *self {
            Shape::Circle(radius) => format!("Circle of radius {}", radius),
            Shape::Square(side) => format!("Square of side {}", side),
            Shape::Rectangle(width, height) => format!("Rectangle of width {} and height {}", width, height),
        }
    }
}

pub fn simulation()
    //where T: std::fmt::Debug
{
    println!("Enum Simulation");
    // create a new enum
    let shape = Shape::Circle(10.1);
    println!("shape: {:?}", shape);
    let shape = Shape::Square(10.2);
    println!("shape: {:?}", shape);
    let shape = Shape::Rectangle(10.1, 10.2);
    println!("shape: {:?}", shape);
    println!();

    // implement the match expression named 'my_shape' for enum 'Shape'
    let my_shape = Shape::Circle(10.1);
    println!("my_shape: {:?}", my_shape);
    // match my_shape {
    //     Shape::Circle(radius) => println!("Circle with radius: {}", radius),
    //     Shape::Square(side) => println!("Square with side: {}", side),
    //     Shape::Rectangle(width, height) => println!("Rectangle with width: {}, height: {}", width, height),
    // }
    println!("Display: {}", my_shape.display());
    println!("Area: {:.2}", my_shape.area());
    println!("Perimeter: {:.2}", my_shape.perimeter());
    println!();

    let my_number = 10u8;
    let result = match my_number { // match should be in sequence
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => {
            println!("Number didn't match: {}", my_number);
            "other"
        },
    };
    println!("result: {}", result);
    println!();

    //
    let number = Some(13);
    // match number {
    //     Some(13) => println!("Thirteen"),
    //     _ => println!("None"),
    // }
    if let Some(13) = number { // here we have replaced match with if let for a simpler case
                                // here we called 'if let' a syntactic sugar
        println!("Thirteen");
    }

    println!();

    let something= Some("One");
    println!("something: {:?}", something);
    //let nothing: Option<T> = None;
    //println!("something: {:?}", nothing);

    println!();
    // create an array of 5 integers
    let numbers = [1, 2, 3, 4, 5];
    println!("numbers: {:?}", numbers);
    let get_number = numbers[2];
    println!("get_number: {}", get_number);
    //let get_last_number = numbers[5]; // this gonna throw error, index out of bounds exception
    let get_last_number = numbers.get(4); // .get() method returns Option<&T> enum with Some and None
    //let inc_number = get_last_number.unwrap() + 1; // .unwrap() is not a good way to get the value, because what if the number is None? If so, then the program panic and crashes
    //let inc_number = get_last_number.unwrap_or(&0) + 1; // solution is to use.unwrap_or(&0) method.
                                                                    // See we added a borrow operator in front of 0 to march the datatype of Option<&T>
    let get_last_number = match get_last_number {
        Some(number) => number + 1,
        None => 0,
    };
    println!("get_last_number: {:?}", get_last_number); // Output: Some(5)

}