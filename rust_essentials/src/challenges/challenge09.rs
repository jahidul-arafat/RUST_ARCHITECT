use std::cmp::Ordering;
use std::fmt;

// create a struct named 'Satellite'
struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

// implement the 'Display' trait for 'Satellite'
impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Implemented Display Trait|| Satellite: {}, velocity: {}", self.name, self.velocity)
    }
}

impl PartialEq<Self> for Satellite {
    fn eq(&self, other: &Self) -> bool {
        self.velocity == other.velocity
    }
}

// implement the 'PartialOrd' trait for 'Satellite'
impl PartialOrd for Satellite { // this gonna implement the super 'PartialOrd' trait
    fn partial_cmp(&self, other: &Satellite) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

// implement the constructor, getter and setter for 'Satellite'
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

pub fn challenge(){
    let mut satellite = Satellite::new("Earth", 1.0);
    println!("{}", satellite); // will call the implemented Display trait
    println!();

    // create two satellites
    let mut sat1 = Satellite::new("Saturn", 2.0);
    let mut sat2 = Satellite::new("Uranus", 2.0);
    println!("{}", sat1); // will call the implemented Display trait
    println!("{}", sat2); // will call the implemented Display trait

    // print if two satellites are equal
    println!("Sat1 == Sat2: {}", sat1 == sat2); // will call the implemented PartialEq trait
}