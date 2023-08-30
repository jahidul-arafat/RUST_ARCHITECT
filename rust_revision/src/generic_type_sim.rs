pub(crate) fn simulation(){
    println!("Generic Type Simulation");
    // create a new rectangle
    let rectangle = Rectangle::new(10u8, 20.2f64);
    println!("{:#?}", rectangle);
    println!("Width: {}", rectangle.get_width());
    println!("Height: {}", rectangle.get_height());
    //println!("Area: {}", rectangle.get_area());// Error, as get_area() method is only defined for Rectangle<u8,u8>, not for Rectangle<T,U>

    let rectangle2 = Rectangle::new(10u8, 20u8);
    println!("{:#?}", rectangle2);
    println!("Width: {}", rectangle2.get_width());
    println!("Area: {}", rectangle2.get_area());

    let rectangle3 = Rectangle::new(20u8, 10.2f64);

    let rect_comparison = is_rectangles_equal(&rectangle, &rectangle3);
    println!("Rectangle comparison (equal): {}", rect_comparison);
    println!("Rectangle comparison (greater): {}", is_rectangle_greater(&rectangle,&rectangle3));
}

// define a struct named Rectangle
// make sure the struct can handle the below data types is a single definition
// f64, i32,u8,u16,u32,f32
// Solution: Use Generic Data Types <T>
// Runtime Performance: Generics are zero cost abstraction, using Monomorphization -> means compiler creates concrete type struct based on the datatype
#[derive(Debug)]
struct Rectangle<T,U> { // is there any way to let Rectangle accept integer types of data too, without modifying the struct definition
    width: T,
    height: U,
}

// impl<T: std::ops::Mul<Output = T>,U> Rectangle<T,U>
// implement the Rectangle
impl<T,U> Rectangle<T,U> {
    fn new(width: T, height: U) -> Rectangle<T,U> {
        Rectangle{
            width,
            height
        }
    }

    // make sure returning slice &T, to avoid transferring ownership
    //fn get_width(self) -> T ; will transfer the ownership of the Rectangle to the get_width function, which is not desired
    fn get_width(&self) -> &T {
        &self.width
    }
    fn get_height(&self) -> &U {
        &self.height
    }
    fn set_width(&mut self, width: T) {
        self.width = width;
    }
    fn set_height(&mut self, height: U) {
        self.height = height;
    }

    // this requires the understanding of Trait
    // fn area(&self) -> T {
    //     self.width * self.height
    // }


}

// now implement the get_area() method for only a specific type of Rectangle
impl Rectangle<u8,u8> {
    // avoid defining the functions already in the the Generic Rectangle implementation; will trigger error
    // fn new(width: u8, height:u8) -> Rectangle<u8,u8> {
    //     Rectangle{
    //         width,
    //         height
    //     }
    // }
    // fn get_width(&self) -> u8 {
    //     self.width
    // }
    fn get_area(&self) -> f64 {
        self.width as f64 * self.height as f64
    }
}

// function to compare two rectangles
// We need the Generic Type T and U the capabilities of >,<,== using PartialOrd trait
fn is_rectangles_equal<T: PartialOrd,U: PartialOrd>(rect1: &Rectangle<T,U>, rect2: &Rectangle<T,U>) -> bool {
    rect1.get_width() == rect2.get_width() && rect1.get_height() == rect2.get_height()
}

fn is_rectangle_greater<T: PartialOrd,U: PartialOrd>(rect1: &Rectangle<T,U>, rect2: &Rectangle<T,U>) -> bool{
    rect1.get_width() > rect2.get_width() && rect1.get_height() > rect2.get_height()
}



