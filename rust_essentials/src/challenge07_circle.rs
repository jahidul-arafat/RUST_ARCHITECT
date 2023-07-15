// define a struct named "Circle"
#[derive(Debug)]
struct Circle {
    radius: f64,
}

// implement the "Area" trait for "Circle"
impl Circle {
    // implement the constructor, getters and setters
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
    fn get_radius(&self) -> f64 {
        self.radius
    }
    fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
    // method to calculate the perimeter
    fn perimeter(&self) -> f64 {
        self.radius * 2.0 * std::f64::consts::PI
    }
    // method to calculate the diameter
    fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    // method to calculate the circumference
    fn circumference(&self) -> f64 {
        self.radius * 2.0 * std::f64::consts::PI
    }
    // method to calculate the volume
    fn volume(&self) -> f64 {
        self.radius * self.radius * self.radius * std::f64::consts::PI
    }
    // method to calculate the surface area
    fn surface_area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
    // method to calculate the inertia
    fn inertia(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
    // method to calculate the moment of inertia
    fn moment_of_inertia(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
    // method to scale the radius
    fn scale(&mut self, factor: f64) {
        println!("Scaling the radius by {}", factor);
        self.radius *= factor;
    }
}

pub fn challenge(){
    let mut circle = Circle::new(5.0);
    println!("Area: {}", circle.area());
    println!("Perimeter: {}", circle.perimeter());
    println!("Diameter: {}", circle.diameter());
    println!("Circumference: {}", circle.circumference());
    println!("Volume: {}", circle.volume());
    println!("Surface area: {}", circle.surface_area());
    println!("Inertia: {}", circle.inertia());
    println!("Moment of inertia: {}", circle.moment_of_inertia());
    println!("");

    circle.scale(2.0);
    println!("Area: {}", circle.area());
    println!("Perimeter: {}", circle.perimeter());
    println!("Diameter: {}", circle.diameter());
    println!("Circumference: {}", circle.circumference());
    println!("Volume: {}", circle.volume());
    println!("Surface area: {}", circle.surface_area());
    println!("Inertia: {}", circle.inertia());
    println!("Moment of inertia: {}", circle.moment_of_inertia());
    println!("");
}