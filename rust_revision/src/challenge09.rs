use std::cmp::Ordering;
use std::fmt;

pub fn challenge(){
    println!("Display Trait");
    let hubble = Satellite{
        name: String::from("Hubble"),
        velocity:12.34
    };
    println!("{}", hubble);
    println!("Altitude {:?}", hubble.altitude());

    let gps = Satellite{
        name: String::from("GPS"),
        velocity:23.45
    };
    println!("{}", gps);
    println!("Altitude {:?}", gps.altitude());

    println!("hubble == gps: {}", hubble == gps); // using PartialEq trait // override based on their velocity
    println!("hubble != gps: {}", hubble != gps); // using PartialEq trait // override based on their velocity
    println!("hubble > gps: {}", hubble > gps); // using PartialOrd trait // override based on their velocity
}

struct Satellite {
    name: String,
    velocity: f64
}
// define a new trait named "Altitude"
trait Altitude {
    fn altitude(&self) -> f64;
}

impl Altitude for Satellite {
    fn altitude(&self) -> f64 {
        self.velocity * 1000.0
    }
}
// implement display trait for Satellite
impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Custom Display Trait)Satellite name: {}, velocity: {}", self.name, self.velocity)
    }
}

// implement PartialEq trait for Satellite to compare two satellites based on their velocity
impl PartialEq for Satellite {
    fn eq(&self, other: &Satellite) -> bool {
        self.velocity == other.velocity
    }
    fn ne(&self, other: &Satellite) -> bool {
        self.velocity!= other.velocity
    }
}

// implement PartialOrd trait for Satellite to compare two satellites based on their velocity
impl PartialOrd for Satellite {
    fn partial_cmp(&self, other: &Satellite) -> Option<Ordering> {
        Some(self.velocity.partial_cmp(&other.velocity)?)
    }
}