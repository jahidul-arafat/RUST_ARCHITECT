use std::ops::Mul;
use std::{cmp, mem};
// Generis doesnt impact the Runtime Performance
// Generics are a zero-cost abstraction, doesnt create additional overhead
// Generics introduces a new concept called 'Monomorphization' -
/*
Monomorphization is a compiler optimization technique used in statically-typed programming languages like Rust.
It is the process of generating specialized versions of generic code by replacing generic types with their concrete
types at compile time.
 */
// define a Struct named 'Rectangle'
// define struct either integer or floating point values
// So, we need to define the struct properties as Generic Type so that it can handle any datatypes i.e. i32, u8,u16,f32,f64 etc
#[derive(Debug)]
struct Rectangle<T,U> {
    width: T, // T--> Generic Type 'T'
    height: U,
}

// implement constructor, getters and setters
impl<T,U> Rectangle<T,U> {
    fn new(width: T, height: U) -> Rectangle<T,U> {
        Rectangle { width, height }
    }

    // method to get width
    fn get_width(&self) -> &T { // see, we are not returning a value, instead we are returning a reference to that value
        // Because, still we dont know the data type 'T'
        // If T is a String, a heap based datatype, then we need to return a reference to that value
        // If T is an integer, a stack based datatype, then we can return that value directly
        &self.width
    }
    // method to get height
    fn get_height(&self) -> &U {
        &self.height
    }
    // method to set width
    fn set_width(&mut self, width: T) {
        self.width = width;
    }
    // method to set height
    fn set_height(&mut self, height: U) {
        self.height = height;
    }

}

// implement constructor, getters and setters for concrete type i.e. u8 type of rectangle
impl Rectangle<u8,u8> {
    fn new_u8(width: u8, height: u8) -> Rectangle<u8,u8> {
        Rectangle { width, height }
    }
    // method to get width
    fn get_width_u8(&self) -> u8 {
        self.width
    }
    // method to calculate the perimeter of rectangle
    fn perimeter(&self) -> u8 {
        self.width * 2 + self.height * 2
    }
}


// define a struct named 'Shuttle' non-generic type
#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

// implement constructor, getters and setters
impl Shuttle {
    fn new(name: &str, crew_size: u8, propellant: f64) -> Shuttle {
        Shuttle {
            name:name.to_string(),
            crew_size,
            propellant
        }
    }
}

pub fn simulation(){
    println!("Simulating Generic Types");
    let mut rectangle = Rectangle::new(10.5, 20); // rust is a strictly typed language
    println!("{:?}", rectangle);


    rectangle.set_width(20.6); // Monomorphization applied
    rectangle.set_height(30);   // Monomorphization applied
    // print the rectangle
    println!("{:?}", rectangle);
    println!();

    // define another rectangle
    let mut rectangle2 = Rectangle::new(10, 20.22);
    println!("{:?}", rectangle2);
    rectangle2.set_width(20);
    rectangle2.set_height(30.0);
    println!("{:?}", rectangle2);

    // define another rectangle
    let mut rectangle3 = Rectangle::new_u8(10u8, 20u8);
    println!("{:?}", rectangle3);
    // print the rectangle perimeter
    println!("Perimeter of rectangle: {}", rectangle3.perimeter());

    // get the biggest of two values
    println!("The biggest value is: {}", get_biggest(10.1,20.2));

    // create a new shuttle
    let mut shuttle = Shuttle::new("Shuttle", 10, 100.0);
    println!("{:?}", shuttle);

    // get the size of memory occupied by the shuttle variable
    println!("The size of memory occupied by the shuttle variable is (STACK): {} bytes", mem::size_of_val(&shuttle)); // 40 bytes
    // get the size of memory occupied by the name variable
    println!("The size of memory occupied by the name variable is: {} bytes", mem::size_of_val(&shuttle.name));
    // get the size of memory occupied by the crew_size variable
    println!("The size of memory occupied by the crew_size variable is: {} bytes", mem::size_of_val(&shuttle.crew_size));
    // get the size of memory occupied by the propellant variable
    println!("The size of memory occupied by the propellant variable is: {} bytes", mem::size_of_val(&shuttle.propellant));
    // get the size of memory occupied by the rest other than the struct variables
    println!("The size of memory occupied by the rest other than the struct variables is: {} bytes",
             mem::size_of_val(&shuttle.name) + mem::size_of_val(&shuttle.crew_size) + mem::size_of_val(&shuttle.propellant));
    println!();
    // BOX is a smart pointer
    /*
    Use cases:
    - Recursion - as whole size cant be known at compile time
    - Transfer ownership of data rather than copying it on the stack to heap; Thus to avoid copying large amount of stack data; Thus performance improved, as copying large amount of data over stack could take time
     */
    // create a new Box with Shuttle type // this will move the Shuttle::new("Shuttle", 10, 100.0) to a HEAP, which was earlier in a STACK
    let shuttle_box = Box::new(shuttle); // not a copy operation // now, shuttle_box will become the owner of Shuttle::new("Shuttle", 10, 100.0) and shuttle will no longer have the ownership
    println!("Shuttle_BOX {:?}", shuttle_box);
    // get the size of memory occupied by the shuttle_box variable
    println!("The size of memory occupied by the shuttle_box variable is (STACK): {} bytes", mem::size_of_val(&shuttle_box)); // 8 bytes //give the size of the box at STACK, which referencing to the shuttle attributes in HEAP
    // get the size of memory occupied by the shuttle_box variable at HEAP // use deref operator to dereference the pointer
    println!("The size of memory occupied by the shuttle_box variable is (HEAP): {} bytes", mem::size_of_val(&*shuttle_box)); // 40 bytes //give the size of the box at HEAP, which referencing to the shuttle attributes in STACK

    // get the size of memory occupied by the name variable
    println!("The size of memory occupied by the name variable is: {} bytes", mem::size_of_val(&shuttle_box.name));
    // get the size of memory occupied by the crew_size variable
    println!("The size of memory occupied by the crew_size variable is: {} bytes", mem::size_of_val(&shuttle_box.crew_size));
    // get the size of memory occupied by the propellant variable
    println!("The size of memory occupied by the propellant variable is: {} bytes", mem::size_of_val(&shuttle_box.propellant));
    println!();

    // move the shuttle_box variable back to the STACK again using deferencing
    let unboxed_shuttle = *shuttle_box;
    println!("Unboxed Shuttle {:?}", unboxed_shuttle);
    // get the size of memory occupied by the shuttle_box variable
    println!("The size of memory occupied by the shuttle_box variable is (STACK): {} bytes", mem::size_of_val(&unboxed_shuttle)); // 8 bytes //give the size of the box at STACK, which referencing to the shuttle attributes in HEAP


}

// method to get the biggest value of two values
// The generic type T is constrained using the PartialOrd trait, which ensures that the values of type T can be compared for ordering.
// The PartialOrd trait represents types that can be partially ordered, meaning they can be compared for ordering but may not have a total ordering defined (e.g., floating-point numbers).
// The Ord trait represents types that have a total ordering defined, meaning they can be compared and have a well-defined order (e.g., integers, characters).
fn get_biggest<T:PartialOrd>(a: T, b: T) -> T {
    // shorthand method to compare a and b
    //cmp::max(a, b)
    if a > b {
        a
    } else {
        b
    }
}



