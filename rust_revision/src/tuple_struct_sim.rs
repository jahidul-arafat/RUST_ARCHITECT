pub(crate) fn simulation() {
    println!("Tuple Struct Simulation");

    // Create a new tuple struct object of Color
    let mut color = Color(255, 255, 255);
    println!("color: {:#?}", color);
    println!("R: {}, G: {}, B: {}", color.0, color.1, color.2);
    color.0 = 0;
    println!("color: {:#?}", color);

    let point1 = Point::new(10,20,30);
    let point2 = Point::new(20,30,40);
    println!("point1: {:#?}", point1);
    println!("point2: {:#?}", point2);
    println!("distance: {:.2}", point1.distance_to(&point2));

    println!("Get Red Color: {:?}",get_red(&color)); // borrowing // not transferring ownership to avoid error
    println!("Get Green Color: {:?}",get_green(&color)); // borrowing // not transferring ownership to avoid error
}

// not a regular struct, but a tuple struct
// define a tuple struct named "Color"
#[derive(Debug)]
struct Color(i32, i32, i32); // RGB

// borrowed Color
fn get_red(color: &Color) -> i32 {
    color.0
}

// borrowed Color
fn get_green(color: &Color) -> i32 {
    color.1
}
// define a tuple struct named "Point"
#[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
    z: i32
}


impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Point {
            x,
            y,
            z
        }
    }
    fn get_x(&self) -> i32 {
        self.x
    }
    fn get_y(&self) -> i32 {
        self.y
    }
    fn get_z(&self) -> i32 {
        self.z
    }
    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        f64::sqrt((dx * dx + dy * dy + dz * dz) as f64)
    }
}

