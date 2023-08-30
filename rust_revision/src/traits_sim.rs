// traits are like interfaces
pub fn simulation() {
    println!("Traits/Interface Simulation");
    let hubble = Satellite {
        name: String::from("Hubble"),
        velocity: 0.0,
    };

    //println!("Satellite {:#?}", hubble); // using debug trait
    println!("{}", hubble.description()); // using Description trait; trait is similar to interface

    let mut gps = Satellite::new("GPS", 2.42);
    gps.set_name("GPS_NEW");
    println!("{}", gps.description());

    // check if two satellites are the equal
    println!("hubble == gps: {}", hubble == gps); // this will trigger error as '==' is not implemented for 'Satellite'
                                                 // Sol: 'Satellite' must implement `PartialEq` trait
    println!("hubble > gps: {}", hubble > gps); // PartialOrd will compare two satellites based on their 'name', then by their velocity;
                                                // if it finds the answer in name, it never compares the velocity

    // what if to compare based on their velocity? not by their 'name'



    let iss = SpaceStation {
        name: String::from("ISS"),
        crew_size: 8,
        altitude: 123,
    };

    //println!("Space Station {:#?}", iss); // using debug trait
    println!("{}", iss.description()); // using Description trait


}

// Trait is similar to interface
// Create a new trait named 'Description'
// Structs need to implement these traits
trait Description {
    //fn description(&self) -> String;
    // trait with default implementation
    fn description(&self) -> String{
        String::from("(Default impl) An object flying through space") // default implementation
    }
}

// implement the trait 'Description' for 'Satellite'
impl Description for Satellite {
    fn description(&self) -> String {   // overriding the default implementation
        format!("(Trait Impl) Satellite name: {}, velocity: {}", self.name, self.velocity)
    }
}

// implement the trait 'Description' for 'SpaceStation'
impl Description for SpaceStation { // this gonna use the default implementation of 'describe' method
    // fn description(&self) -> String { // overriding the default implementation
    //     format!("(Trait Impl) Space Station name: {}, crew size: {}, altitude: {}", self.name, self.crew_size, self.altitude)
    // }
}

// by default new struct doesnt implement any trait
// create a Struct named "Satellite"
#[derive(Debug)] // debug trait for nicely formatted output
#[derive(PartialEq,PartialOrd)] // PartialEq trait for checking if two objects are equal
struct Satellite {
    name: String,
    // heap
    velocity: f64, // stack
}

// implement the struct 'Satellite'
impl Satellite {
    fn new(name: &str, velocity: f64) -> Satellite {
        Satellite {
            name: String::from(name),
            velocity,
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
    fn get_velocity(&self) -> f64 {
        self.velocity
    }
    fn set_velocity(&mut self, velocity: f64) {
        self.velocity = velocity;
    }
    fn description(&self) -> String { // this will override the trait implementation for 'description' method
        format!("(Inside method)Satellite name: {}, velocity: {}", self.name, self.velocity)
    }

}



// by default new struct doesnt implement any trait
//#[derive(Debug)] // debug trait for nicely formatted output
struct SpaceStation {
    name: String,
    // @heap
    crew_size: u32,
    // @stack
    altitude: u32, // @stack
}