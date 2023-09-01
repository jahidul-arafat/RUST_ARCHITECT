/* Not Allowed
    let ref1 = &mut var;
    let ref2 = &var; // can't re-reference an already borrowed mutable variable to avoid data races in multi threading
     */
pub fn simulation(){
    println!("Lifetime of Struct");
    let mut shuttle1 = Shuttle::new("Shuttle1", 20,1234.56);
    shuttle1.print_info();
    let convergent = String::from("c");
    println!("Lifetime Elision Rule Testing :{}", shuttle1.lifetime_test_method(&convergent));

    println!("Lifetime of Struct/Send Transmission: {}",shuttle1.send_transmission_with_lf_annotation("MSG RETURN"));
    println!("Lifetime of Struct/Send Transmission: {}",shuttle1.send_transmission_without_lf_annotation("NAME RETURN"));
}

// create a new struct named "Shuttle"
// implement the debug trait for the struct
// struct is a stack-only datatype
// its like a Class in Java
#[derive(Debug)]
struct Shuttle<'a> {
    //name: String,
    // name: &str   // this will throw error //missing lifetime specifier
    //                 // why not working without explicit lifetime annotation?
    //                 // because struct no longer owns the variable 'name', its a reference instead
    //                 // so, its not clear how the lifetime of the borrowed str related to the lifetime of the Shuttle struct
    //                 // if the str 'name' is dropped or disappeared, struct is still onscope and still referencing a no-longer existing string, then that will be a problem
    //                 // Solution: Explicitly specify the lifetime annotation of the variable 'name' for the struct related to the str-slice its storing
    name: &'a str,
    crew_size: usize,
    propellant: f64
}

// implement the struct
impl<'a> Shuttle<'a> {
    // Constructor //Associated function
    fn new(name: &str, crew_size: usize, propellant: f64) -> Shuttle {
        Shuttle {
            //name: String::from(name), // @heap, pointer @stack
            name,
            crew_size,  // @stack
            propellant  // @stack
        }
    }

    fn get_name(&self) -> &str {
        //&self.name // as name is a str slice already, not String
        self.name
    }

    fn get_crew_size(&self) -> usize {
        self.crew_size
    }

    fn get_propellant(&self) -> f64 {
        self.propellant
    }

    fn set_propellant(&mut self, propellant: f64) {
        self.propellant = propellant;
    }

    fn set_name(&mut self, name: &'a str) { // name is a str slice, borrowed, not owner by Struct, that's why lifetime specifier is needed
        //self.name = String::from(name);
        self.name = name; // as name is a str slice already, not String
    }
    fn scale_speed(&mut self, factor: f64) {
        self.propellant = self.propellant * factor;
    }

    // fn lifetime_test_method<'a,'b>(&'a self, convergent: &'b str) -> &'a str
    fn lifetime_test_method(&self, convergent: &str) -> &str {
        if self.name.len()>4 && self.crew_size > 10 && self.propellant > 1000.0 && matches!(convergent,"c"|"conv"){
            return "Shuttle is alive";
        }
        "not convergent"
    }

    // msg is borrowed, Struct doesnt own it; so we have to specify lifetime specifier for msg (because it is returned),
    // that its in the same lifetime of the Struct to avoid dangling references
    // if msg was not returned, then no lifetime annotation would be needed
    fn send_transmission_with_lf_annotation(&'a self, msg: &'a str) -> &str {
        println!("Sending message: {}", msg);
        let mut new_msg:String =String::from("New Name");
        new_msg.push_str(&msg);
        msg
    }

    // lifetime elision rule applied
    // no explicit lifetime annotation needed
    fn send_transmission_without_lf_annotation(&self, msg: &str) -> &str {
        println!("Sending message: {}", msg);
        self.name
    }

    fn print_info(&self) {
        println!("The Shuttle {} has {} crews and flying at speed {}", self.name, self.crew_size, self.propellant);
    }
}