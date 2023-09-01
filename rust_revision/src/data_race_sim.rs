pub(crate) fn simulation(){
    println!("Data Race Simulation");
    /* Not Allowed
    let ref1 = &mut var;
    let ref2 = &var; // can't re-reference an already borrowed mutable variable to avoid data races in multi threading
     */
    let mut rocket_fuel = String::from("RP-1");

    rocket_fuel = produce_fuel_avoid_dangling_reference(); // get the out of scopred new_fuel
    println!("Rocket Fuel: {}", rocket_fuel);

    let mut length = produce_fuel_ownership_borrow_mutable(&mut rocket_fuel);
    println!("Rocket Fuel: {} Rocket Length: {}", rocket_fuel,length);

    length = produce_fuel_ownership_borrow_nonmutable(&rocket_fuel);
    println!("Rocket Fuel: {} Rocket Length: {}", rocket_fuel,length);
}

// returns a reference
// fn produce_fuel_with_dangling_reference()-> &String{
//     let new_fuel = String::from("NEW-FUEL");
//     &new_fuel // this goes out of scope when returned // example of Dangling reference
// }

fn produce_fuel_avoid_dangling_reference()-> String{
    let new_fuel = String::from("NEW-FUEL");
    new_fuel
}

fn produce_fuel_ownership_borrow_nonmutable(rocket_fuel: &String)-> usize{
    rocket_fuel.len()
}

fn produce_fuel_ownership_borrow_mutable(rocket_fuel: &mut String)-> usize{
    rocket_fuel.push_str("--appended--");
    rocket_fuel.len()
}

