pub(crate) fn simulation() {
    // transfer_ownership_v1();
    // transfer_ownership_v2();
    //string_stack_heap();
    let mut rocket_fuel = String::from("RP-1"); // string at heap
    rocket_fuel = process_fuel_string(rocket_fuel); // ownership transferred of heap to the function
    println!("Rocker Fuel after function call: {:?}", rocket_fuel); // rocker_fuel variable no longer in scope
}

fn transfer_ownership_v1() {
    //let x = "My String";
    let mut x = String::from("My String");
    println!("x = {}", x);
    {
        let mut y = &mut x; // mutable borrowing
        *y = String::from("My New String"); // dereferencing // assigning value to y
        println!("y = {}", y);
    }
    println!("x = {}", x); // will fail here as ownership is already transferred to y
}

fn transfer_ownership_v2() {
    let mut x = "Old Me";
    println!("x = {}", x);
    {
        let mut y = &mut x; // ownership transferred to y
        *y = "New Me"; // dereferencing // assigning value to y
        println!("y = {}", y);
    }
    println!("x = {}", x); // New Me
}

fn string_stack_heap() {
    // heap has plenty of space, but not infinite space
    let mut message = String::from("Earth");
    message.push_str(" is home");
    println!("message = {}, length={}, capacity={}", message, message.len(), message.capacity());
    message.clear(); // deallocating the heap memory
    println!("message = {}, length={}, capacity={}", message, message.len(), message.capacity());
}

fn process_fuel_integer(mut fuel: i32) {
    fuel += 100;
    println!("fuel inside the function call = {}", fuel);
}

fn process_fuel_string(mut fuel: String) -> String {
    println!("Original Fuel Received: {}", fuel);
    fuel = String::from("RP-NEW-FUEL");
    fuel.push_str("-A-DUMMY-FUEL-");
    println!("fuel inside the function call = {}", fuel);
    String::from("LPG") // returning the ownership to the original variable
}