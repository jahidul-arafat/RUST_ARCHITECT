// Special Purpose Lifetime annotation: 'static
// this indicates reference variables available for entire duration of the program and at any time
pub fn simulation(){
    println!("Static Lifetime Annotation &'static");
    let msg: &'static str = "Static Lifetime Annotation";
    let person1 = Person::new("Alice",20);
    println!("{:?}", person1);
    println!("{}",person1.static_method(msg));
}


#[derive(Debug)] // debug trait for printing
struct Person {
    name: &'static str, // its borrowed
    age: u8,
}

impl Person {
    fn new(name: &'static str, age: u8) -> Person {
        Person {
            name,
            age,
        }
    }

    fn static_method(&self, msg:&'static str) -> &str {
        println!("Sending message: {}", msg);
        let new_msg =String::from("NEW_MSG");
        //self.name
        msg
    }
}