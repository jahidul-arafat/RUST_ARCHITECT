// define a Struct named "Rectangle"
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64
}

// implement the constructor, getters and setters
impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {
            width: width,
            height: height
        }
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        self.width * 2.0 + self.height * 2.0
    }
    fn scale(&mut self, factor: f64) {
        println!("Scaling rectangle by factor {}", factor);
        self.width *= factor;
        self.height *= factor;
    }
    fn set_width(&mut self, width: f64) {
        self.width = width;
    }
    fn set_height(&mut self, height: f64) {
        self.height = height;
    }
    fn get_width(&self) -> f64 {
        self.width
    }
    fn get_height(&self) -> f64 {
        self.height
    }
}

pub fn challenge(){
    println!("Challenge 07/Struct and Tuple");
    let mut rectangle = Rectangle::new(10.0, 20.0);
    println!("Rectangle: {:?}", rectangle);
    println!("Area: {}", rectangle.area());
    println!("Perimeter: {}", rectangle.perimeter());
    println!("Width: {}", rectangle.get_width());
    println!("Height: {}", rectangle.get_height());
    println!();
    rectangle.scale(2.0);
    println!("Rectangle: {:?}", rectangle);

    rectangle.set_width(20.0);
    rectangle.set_height(30.0);
    println!("Rectangle: {:?}", rectangle);
    println!("Area: {}", rectangle.area());
    println!("Perimeter: {}", rectangle.perimeter());
    println!("Width: {}", rectangle.get_width());
    println!("Height: {}", rectangle.get_height());
}
