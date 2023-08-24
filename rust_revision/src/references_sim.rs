pub(crate) fn simulation() {
    println!("Referencing Simulation... Without transferring ownership");
    let mut rocket_fuel = String::from("RP-1");
    println!("Rocket Fuel: {}", rocket_fuel);
    //let length = process_fuel_ownership_noroundtrip(rocket_fuel); // returns ownership to rocket_fuel
    //println!("Rocket Fuel: {} and Length: {}", rocket_fuel, length); // will throw error as the rocket_fuel ownership is transferred to the function and it didnt return the ownership

    //let (rocket_fuel,length) = process_fuel_ownership_roundtrip(rocket_fuel); // ownership returned to avoid the error
    //println!("Rocket Fuel: {} and Length: {}", rocket_fuel, length);

    let length = process_fuel_ownership_borrow_then_modify(&mut rocket_fuel); // returns ownership to
    println!("Rocket Fuel: {} and Length: {}", rocket_fuel, length);

    let length = process_fuel_ownership_borrow_dont_modify(&rocket_fuel); // returns ownership to
    println!("Rocket Fuel: {} and Length: {}", rocket_fuel, length);


}

fn process_fuel_ownership_noroundtrip(rocket_fuel: String) -> usize {
    println!("Processing rocket fuel...{}", rocket_fuel);
    //rocket_fuel = String::from("LPG");
    let length = rocket_fuel.len();
    //(rocket_fuel, length) // see ownership of rocket_fuel is making a round trip here which is cumbersome and not desirable
    length
}

fn process_fuel_ownership_roundtrip(mut rocket_fuel: String) -> (String, usize) {
    println!("Processing rocket fuel...{}", rocket_fuel);
    rocket_fuel = String::from("LPG");
    let length = rocket_fuel.len();
    (rocket_fuel, length) // see ownership of rocket_fuel is making a round trip here which is cumbersome and not desirable
}

/*
&String -> means borrowing a reference to the String
&mut String -> means reference we are borrowing is mutable
 */

fn process_fuel_ownership_borrow_dont_modify(rocket_fuel: &String)-> usize {
    println!("Inside borrow dont modify");
    println!("Processing rocket fuel...{}", rocket_fuel);
    //rocket_fuel.push_str("--attempt-to-modify"); // cant modify as the rocket_fuel is an immutable borrow // sol: use &mut String --> to convert it to be a mutable borrow
    let length = rocket_fuel.len();
    length
}

fn process_fuel_ownership_borrow_then_modify(rocket_fuel: &mut String) -> usize{
    println!("Inside borrow then modify");
    println!("Processing rocket fuel...{}", rocket_fuel);
    // modify the borrowed rocket_fuel
    rocket_fuel.push_str("--a-modification"); // this will be modifying the original rocket_fuel variable
    let length = rocket_fuel.len();
    length
}