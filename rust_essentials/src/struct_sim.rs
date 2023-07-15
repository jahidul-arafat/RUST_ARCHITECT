/**
- Struct data will always be stored in the STACK memory, unless explicitly mentioned to be in HEAP.
*/
#[derive(Debug)] // to avoid using it, we have to define Traits
struct Point {
    x: f32,
    y: f32,
}

// define constructor, getters and setters for Point struct
impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
    fn get_x(&self) -> f32 {
        self.x
    }
    fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    fn get_y(&self) -> f32 {
        self.y
    }
    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

}

#[derive(Debug)]
#[derive(Clone)]    // to avoid the String issues in the Shuttle struct when copying the data from vehicle to vehicle3
struct Shuttle{
    name: String,
    crew_size: u8,
    propellant: f64
}

// Define a tuple struct named "Color"
#[derive(Debug)]
struct Color(u8, u8, u8); //RGB

// Define a tuple struct named "Triangle"
#[derive(Debug)]
struct Triangle(f32, f32, f32); //XYZ

fn get_triangle_y(t: Triangle) -> f32 {
    t.1
}
/**
Define 3 methods for the Shuttle struct.
Methods are:
- get_name
- get_crew_size
- get_propellant
*/
// define a method 'get_name' for the Shuttle struct
impl Shuttle{
    // method to initialize the Shuttle struct
    fn new(name: &str, crew_size: u8, propellant: f64) -> Self{
        Shuttle{
            name: name.to_string(),
            crew_size: crew_size,
            propellant: propellant
        }
    }

    fn get_name(&self) -> &str{
        &self.name
    }
    fn get_crew_size(&self) -> u8{
        self.crew_size
    }
    fn get_propellant(&self) -> f64{
        self.propellant
    }
    fn set_name(&mut self, name: &str){
        println!("Setting the name to {}", name);
        self.name = name.to_string();
    }
    fn set_crew_size(&mut self, crew_size: u8){
        println!("Setting crew size to {}", crew_size);
        self.crew_size = crew_size;
    }
    fn set_propellant(&mut self, propellant: f64){
        println!("Setting propellant to {}", propellant);
        self.propellant = propellant;
    }
    fn add_fuel(&mut self, amount: f64){
        println!("Adding fuel {} to the shuttle",amount);
        self.propellant += amount;
    }
}


pub fn simulation (){
    // create 10 points
    // let mut points = Vec::new();
    // for i in 0..10 {
    //     points.push(Point { x: i as f32, y: i as f32 });
    // }
    // // sort the points
    // points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    // println!("{:?}", points);
    //
    // let point = Point { x: 2.5, y: 1.0 };
    // println!("Point coordinates: ({}, {})", point.x, point.y);

    let mut vehicle = Shuttle{
        name: String::from("Shuttle"), // to HEAP
        crew_size: 7, // to STACK
        propellant: 8444.4345   // to STACK
    };

    let mut vehicle2 = Shuttle{
        name: String::from("Shuttle2"), // to HEAP
        ..vehicle // copy the rest of the data from vehicle to vehicle2
    };

    // Will throw ERROR
    let mut vehicle3 = Shuttle{
        ..vehicle.clone()
        //..vehicle // copy all the data from vehicle to vehicle3 // will throw "COMPILER ERROR"
        // this happens because, now vehicle3 takes the ownership of string in vehicle.name
        // once it is done, vehicle.name is no longer valid and would fail to compile

        // solution is to clone the vehicle to vehicle3

    };

    let mut vehicle4 = Shuttle::new("Shuttle4", 22, 1234.4345);


    // Printing the info
    println!("{:?}", vehicle);
    println!("Vehicle Name: {:?}", vehicle.get_name());
    println!("Vehicle Crew Size: {:?}", vehicle.get_crew_size());
    println!("Vehicle Propellant: {:?}", vehicle.get_propellant());
    vehicle.set_name("New Shuttle3");
    vehicle.add_fuel(1000.0);
    println!("Vehicle Name: {:?}", vehicle.get_name());
    println!("Fuel Updated: {:?}", vehicle.get_propellant());

    // println!("{:?}", vehicle.name);
    // println!("{:?}", vehicle.crew_size);
    // println!("{:?}", vehicle.propellant);
    // vehicle.crew_size = 10;
    // println!("{:?}", vehicle.crew_size);

    println!("{:?}", vehicle2);
    println!("{:?}", vehicle3);

    println!();
    println!("{:?}", vehicle4);
    println!();

    let mut tuple1 =(1,"arafat",32.32);
    //tuple1.3="one two three";
    println!("{:?}", tuple1);
    println!("{:?}", tuple1.0);
    println!("{:?}", tuple1.1);
    println!("{:?}", tuple1.2);
    //println!("{:?}", tuple1.3);

    // destructure the tuple
    let (x,y,z) = tuple1;
    println!("{:?}", x);

    // create a tuple of 3 Shuttles
    // tuples are fixed sized, cant be added new values later
    let mut tuple2 = (vehicle, vehicle2, vehicle3);
    // add a new Shuttle to the tuple
    //tuple2.3=vehicle4;
    tuple2.0=vehicle4;

    println!("{:?}", tuple2.0);

    println!();
    println!("Simulating Tuple Struct DataType\n------------------------------");
    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let blue = Color(0, 0, 255);
    println!("Red: {:?}, Green: {:?}, Blue: {:?}", red, green,blue);
    println!("Red: {:?}, Green: {:?}, Blue: {:?}", red.0, green.0,blue.0);

    println!();
    let triangle = Triangle(1.0, 20.0, 3.0);
    println!("Triangle: {:?}", triangle);
    println!("Triangle Y: {:?}", get_triangle_y(triangle));
}
