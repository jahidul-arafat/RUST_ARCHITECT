// list all the derivable traits
/*
Eq
PartialEq
Ord
PartialOrd
Clone
Copy
Hash
Default
Debug
 */

// Traits are similar to interfaces in Java.
// They allow you to define a set of methods that a type must implement.
// define a struct named 'Satellite'
#[derive(Debug)] // Derivable Trait -->Compiler will generate default code for the required methods; Debug is a trait, that Satellite implements.
#[derive(PartialEq, PartialOrd)] // PartialEq is a trait, that Satellite implements
struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

// implement the constructor, getter and setter methods
impl Satellite {
    fn new(name: &str, velocity: f64) -> Satellite {
        Satellite {
            name: name.to_string(),
            velocity
        }
    }
    fn get_velocity(&self) -> f64 {
        self.velocity
    }
    fn set_velocity(&mut self, velocity: f64) {
        self.velocity = velocity;
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

// define a struct named 'SpaceStation'
#[derive(Debug)]
#[derive(PartialEq)]
struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32 //miles
}
// implement the constructor, getter and setter methods
impl SpaceStation {
    fn new(name: &str, crew_size: u8, altitude: u32) -> SpaceStation {
        SpaceStation {
            name: name.to_string(),
            crew_size,
            altitude
        }
    }
    fn get_altitude(&self) -> u32 {
        self.altitude
    }
    fn set_altitude(&mut self, altitude: u32) {
        self.altitude = altitude;
    }
    fn get_crew_size(&self) -> u8 {
        self.crew_size
    }
    fn set_crew_size(&mut self, crew_size: u8) {
        self.crew_size = crew_size;
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

// Traits are similar to interfaces in Java.
// create a trait named 'Description' to describe the Struct
trait Description {
    // now we have to implement this method in the implementation blocks of the Struct
    fn describe(&self) -> String {
        // default implementation
        String::from("An object flying through space!!") // the individually implemented describe will override this
    }

}

trait ComparisonSatellite {
    fn greater_than(&self, other: &Satellite) -> bool;

}

trait ComparisonSpaceStation {
    fn greater_than(&self, other: &SpaceStation) -> bool;
}

// implement the trait for the struct 'Satellite'
impl Description for Satellite {
    // we comment this implementation, now the satellite.describe() will use the default implementation
    // fn describe(&self) -> String {
    //     format!("Satellite {} has a velocity of {}", self.name, self.velocity)
    // }
}

// implement the Comparison trait for the struct 'Satellite'
impl ComparisonSatellite for Satellite {
    fn greater_than(&self, other: &Satellite) -> bool {
        self.velocity > other.velocity
    }
}

// implement the trait for the struct 'SpaceStation'
impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("Space Station {} has a crew size of {} and an altitude of {}", self.name, self.crew_size, self.altitude)
    }
}

// implement the Comparison trait for the struct 'SpaceStation'
impl ComparisonSpaceStation for SpaceStation {
    fn greater_than(&self, other: &SpaceStation) -> bool {
        // compare the crew size and altitude
        self.crew_size > other.crew_size && self.altitude > other.altitude
    }
}

pub fn simulation(){
    println!("Simulating Traits...");

    // create a satellite
    let mut satellite = Satellite::new("Earth", 10.0);
    println!("Satellite: {:?}", satellite);
    // describe the satellite
    println!("Satellite description: {}", satellite.describe());
    println!();

    // create a space station
    let mut space_station = SpaceStation::new("Earth", 10, 1000);
    println!("Space Station: {:?}", space_station);
    // describe the space station
    println!("Space Station description: {}", space_station.describe());

    // create two additional satellites
    let mut satellite2 = Satellite::new("Hubble Telescope", 50.0);
    let mut satellite3 = Satellite::new("GPS", 10.0);
    // print if the satellites are equal
    println!("Satellites are equal: {}", satellite2 == satellite3); // Error: binary operation `==` cannot be applied to type `Satellite`
                                                                    // note: an implementation of `PartialEq<_>` might be missing for `Satellite`
                                                                    // Solution: #[derive(PartialEq)]
    println!("satellite2>satellite3 (Default Trait Impl): {}", satellite2 > satellite3); // Error: binary operation `>` cannot be applied to
                                                                    // Sol: Derive PartialOrd trait
                                                                    // What's they are comparing?
    // calculate the ASCII value of the satellite name
    println!();
    let total_value_sat2_name: u32 = satellite2.get_name().chars().map(|c| c as u32).sum(); // 403
    let total_value_sat3_name: u32 = satellite3.get_name().chars().map(|c| c as u32).sum(); // 739
    println!("total_value_sat2_name: {}", total_value_sat2_name);
    println!("total_value_sat3_name: {}", total_value_sat3_name);
    println!("Name ASCII comparison: {:?}",total_value_sat2_name>total_value_sat3_name);
    println!("Velocity comparison: {:?}",satellite2.get_velocity()>satellite3.get_velocity());
    println!();

    // check if satellite 2 is greater than satellite 3
    println!("Satellite 2>Satellite 3 (Custom Trait Impl): {}", satellite2.greater_than(&satellite3));

    // define two space stations
    let mut space_station2 = SpaceStation::new("Mars", 20, 1000);
    let mut space_station3 = SpaceStation::new("Jupiter", 10, 2000);
    // print if the space stations are equal
    println!("Space Stations are equal: {}", space_station2 == space_station3);
    println!("Space Station 2>Space Station 3:{}",space_station2.greater_than(&space_station3));

    



}