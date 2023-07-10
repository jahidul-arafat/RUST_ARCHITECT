use std::io;

pub fn simulation() {
    println!("Variable Scope and Ownership");

    // basic();
    // let user_input = take_input();
    // println!("user_input = {}", user_input);
    //
    // move_ownership_string();
    //copy_onwership_integer();
    //fuel_simulator();
    fuel_simulator2();
    //no_ownership();
}

// this should fail
fn no_ownership() {
    let x = String::from("My String");
    println!("x = {}", x);
    {
        let y = x;
        println!("y = {}", y);
    }
    println!("x = {}", x);
}

fn basic() {
    let x = 5;
    {
        let x = 'C';
        let mut name = "Arafat"; // name is a reference to the actual string value.
        // It points to the memory location where the string data is stored.
        println!("The value of name is {}", name);

        name = "Jahidul";
        println!("The value of name is {}", name);
        if true {
            println!("x = {}", x);
        }
        println!("x = {}", x);
    }
    println!("x = {}", x);
}

/*
Program Memory
(a) Stack - LIFO; small; cant store tons of data in Stack. We many not always know how much memory will be needed
- Stack only datatypes are: integer and floating point
(b) Heap - large enough memory to store tons of data. Less organized. Elements can be any where.
Pointer - stores the memory address. Follow the pointer, makes accessing data in heap is slower than the stack
- Heap datatypes are: String
 */

// method to take input from user
fn take_input() -> String {
    let mut planet = String::from("Earth");
    planet.push_str(" is the largest planet in the solar system.");
    println!("{},{}", planet, planet.len());

    println!("Enter your name: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line"); // input to be mutable for the runtime. Heap implementation
    println!("You entered: {}", input);
    input
}

fn move_ownership_string() {
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mars");
        // outer_planet = inner_planet; // shallow copy ; dereferencing the inner_planet; inner_planet will be affected
        outer_planet = inner_planet.clone(); // deep copy // creates another heap memory location
        inner_planet.clear(); // heap memory content is freed
        println!("inner_planet = {}", inner_planet); // print empty
        //outer_planet = inner_planet;
    }
    println!("outer_planet = {}", outer_planet);
}

fn copy_onwership_integer() {
    let outer_planet: i32;
    {
        let mut inner_planet = 100; // this data lives in a stack; doesnt reference anything in the heap
        outer_planet = inner_planet;
        inner_planet += 1;
        println!("inner_planet = {}", inner_planet);
    }
    println!("outer_planet = {}", outer_planet);
}

fn fuel_simulator() {
    let rocket_fuel = 1;
    println!("rocket_fuel = {}", rocket_fuel);
    process_fuel(rocket_fuel);
    println!("rocket_fuel = {}", rocket_fuel);
    //let after_processing_fuel = process_fuel(rocket_fuel);
    //println!("Before processing: {}, after_processing_fuel = {}", rocket_fuel, after_processing_fuel);
}

fn process_fuel(mut propellant: i32) {
    propellant = propellant * 10;
    println!("processing propellant = {} ...", propellant);
}

fn fuel_simulator2() {
    let mut rocket_fuel = String::from("1");
    println!("rocket_fuel = {}", rocket_fuel);
    let rocket_fuel = process_fuel2(rocket_fuel); // heap reference // with .clone() we are creating another heap memory location
    println!("rocket_fuel = {}", rocket_fuel); // got the modified value at the heap reference
}

fn process_fuel2(mut propellant: String) -> String {
    propellant = propellant + " XYZ";
    println!("processing propellant = {}...", propellant);
    propellant
}