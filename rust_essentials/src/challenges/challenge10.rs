use std::fmt;

// define an enum named 'Location' with 3 variants: Unknown, Anonymous and Known with latitude and longitude stored in float values
enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64)
}

// create an implementation of Location
impl Location {
    fn new(latitude: f64, longitude: f64) -> Location {
        Location::Known(latitude, longitude)
    }
    fn get_latitude(&self) -> f64 {
        match self {
            Location::Unknown => 0.0,
            Location::Anonymous => 0.0,
            Location::Known(latitude, _) => *latitude
        }
    }
    fn get_longitude(&self) -> f64 {
        match self {
            Location::Unknown => 0.0,
            Location::Anonymous => 0.0,
            Location::Known(_, longitude) => *longitude
        }
    }
    fn is_anonymous(&self) -> bool {
        match self {
            Location::Unknown => true,
            Location::Anonymous => true,
            Location::Known(_, _) => false
        }
    }
    fn is_known(&self) -> bool {
        match self {
            Location::Unknown => false,
            Location::Anonymous => false,
            Location::Known(_, _) => true
        }
    }

    // method to implement Display
    fn display(&self) -> String {
        match self {
            Location::Unknown => "Unknown".to_string(),
            Location::Anonymous => "Anonymous".to_string(),
            Location::Known(latitude, longitude) => format!("Known Location: {}, {}", latitude, longitude)
        }
    }
}

// implement Display trait for Location
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Known Location/Using Display Trait: {}", self.display())
    }
}


pub fn challenge() {
    println!("RUST enum challenge");
    let location = Location::new(37.422, -122.084);
    println!("Location: {}", location.display());
    println!("Location is anonymous: {}", location.is_anonymous());
    println!("Location is known: {}", location.is_known());

    // print using the display trait
    println!("Location/Using Display Trait: {}", location);


    let location = Location::Anonymous;
    println!("Location: {}", location.display());

    let location = Location::Known(37.422, -122.084);
    println!("Location: {}", location.display());


}