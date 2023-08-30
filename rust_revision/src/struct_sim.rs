pub(crate) fn simulation(){
    println!("Struct Simulation");
    // create a new instance of the struct Shuttle
    let mut vehicle = Shuttle{
        name: String::from("Shuttle"),
        crew_size: 7,
        propellant: 1234.45
    };
    println!("{:#?}", vehicle);
    let vehicle2 = Shuttle{
        ..vehicle.clone()
    };
    println!("{:#?}", vehicle2);
    println!("{:#?}", vehicle); // will trigger error as String could have only one owner and ownership transferred to vehicle2 already

    let mut vehicle3 = Shuttle::new("Shuttle3", 7, 1234.45);
    vehicle3.set_name("Shuttle3X");
    vehicle3.print_info();
    vehicle3.add_fuel(100.1);
    vehicle3.print_info();

}

// create a new struct named Shuttle
// Struct is like a class in Java
#[derive(Debug)] // debug trait is used// this is a custom datatype, so default formatter will not work. We need debug formatting -> but debug formatter itself cant resolve the problem
#[derive(Clone)] // clone trait for Shuttle
struct Shuttle { // Struct is Stack-only datatype
    name: String, // ptr in Stack, but Data is in Heap // is &str--> string slice, then we need a lifetime annotation
    crew_size: u8, // is in Stack
    propellant: f64 // is in Stack
}

// implement Shuttle
impl Shuttle {
    // constructor // associated function
    // cant have multiple constructors like in Java
    fn new(name: &str, crew_size: u8, propellant: f64) -> Self {
        Shuttle {
            name: String::from(name),
            crew_size,
            propellant
        }
    }

    // create another constructor
    // fn new(name: &str) -> Self {
    //     Shuttle {
    //         name: String::from(name),
    //         crew_size: 7,
    //         propellant: 1234.45
    //     }
    // }
    // return type -> slice
    // &-> borrow operator
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_crew_size(&self) -> u8 {
        self.crew_size
    }
    fn get_propellant(&self) -> f64 {
        self.propellant
    }
    fn set_crew_size(&mut self, crew_size: u8) {
        self.crew_size = crew_size;
    }
    fn set_propellant(&mut self, propellant: f64) {
        self.propellant = propellant;
    }
    fn set_name(&mut self, name: &str){
        self.name = String::from(name);
    }
    fn print_info(&self) {
        println!("{:#?}", self);
    }
    fn add_fuel(&mut self, amount: f64) {
        self.propellant+=amount;
    }
}