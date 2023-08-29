pub(crate) fn challenge(){
    println!("Rectangle Struct!");
    let mut rectangle = Rectangle::new(11.2,13.0);
    rectangle.print();
    println!("Rectangle Area: {}",rectangle.get_area());
    rectangle.scale(10.0);
    rectangle.print();
}

// define a Rectangle struct
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// implement the Rectangle struct
impl Rectangle {
    // constructor //associated function
    fn new(width: f64, height: f64) -> Self {
        Rectangle {
            width,
            height,
        }
    }
    fn get_width(&self)->f64{
        self.width
    }
    fn get_height(&self)->f64{
        self.height
    }
    fn get_area(&self) -> f64 {
        self.width * self.height
    }
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
    fn perimeter(&self) -> f64 {
        self.width * 2.0 + self.height * 2.0
    }
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    fn print(&self) {
        println!("width: {}, height: {}", self.width, self.height);
    }
}