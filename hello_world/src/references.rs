pub fn simulation() {
    println!("Dont Transfer Ownership of Data, Let them Borrow and return back after a while");
    println!("This is like a Car Rental System, When I rent the car, I can use, but I cant modify or sell it as I am not the owner");

    //fuel_sim();
    //fuel_sim_borrow();
    fuel_sim_borrow_data_race();
}

fn fuel_sim_ownership_transfer() {
    let rocket_fuel = String::from("RP-1");
    println!("Before Processing: {}", rocket_fuel);

    let rocket_fuel = process_fuel_ownership_transfer(rocket_fuel); // DrawbackL rocket_fuel is doing a roundtrip here, which could be avoided
    println!("After Processing: {},{}", rocket_fuel.0, rocket_fuel.1);
}

fn process_fuel_ownership_transfer(fuel: String) -> (i32, String) {
    println!("Processing Fuel");
    let length = fuel.len() as i32;
    println!("Length of Fuel: {}", length);
    (length, String::from("LPG"))
}

/*
1 Mutable Reference // ALLOWED in a scope
-----------------
let ref1 = &mut var;

2+ Immutable Reference // ALLOWED in a scope
---------------------
let ref1 = &var;
let ref2 = &var;

1 Mutable Reference + Any other Reference // NOT ALLOWED in a scope
------------------------------------------------
let ref1 = &mut var;
let ref2 = &var;


 */
fn fuel_sim_borrow() {
    let mut rocket_fuel = String::from("RP-1");
    println!("Before Processing: {}", rocket_fuel);

    // using mutual reference
    // Restriction: When using a mutable reference, you cant create other references of the same variable
    // Why: to avoid data races
    let mutable_ref = &mut rocket_fuel;
    let fuel_length = process_fuel_borrowing_a_reference(mutable_ref); // &rocket_fuel --> means let the function to borrow rocket_fuel parameter, ownership of rocket_fuel is not transferred to the function
    println!("After Processing: {},{}", fuel_length, rocket_fuel);

    // lets check the data_race
    let immutable_ref = &rocket_fuel; // can we do it!!!!
    let fuel_length_2 = process_fuel_borrowing_a_reference2(immutable_ref);
    println!("After Processing// Checking Data_race: {},{}", fuel_length_2, rocket_fuel);
}

/*
&String -> means borrowing a reference to the String
&mut String -> means reference we are borrowing is mutable
 */
fn process_fuel_borrowing_a_reference(fuel: &mut String) -> i32 {
    fuel.push_str(" is highly flammable");
    println!("Processing Fuel: {}", fuel);
    let length = fuel.len() as i32;
    println!("Length of Fuel: {}", length);
    length
}

fn process_fuel_borrowing_a_reference2(fuel: &String) -> usize {
    println!("For Data Race Testing....");
    //fuel.push_str(" is highly cool"); // see, rocket_fuel in the heap is going to be modified
    println!("Processing Fuel: {}", fuel);
    fuel.len() * 100
}


fn fuel_sim_borrow_data_race() {
    // will not cause data race, as we need to define 'Threads' and let the threads pick the process_fuel_borrowing_a_reference_data_race function
    // here, things will run sequentially
    let mut rocket_fuel = String::from("RP-1");
    println!("Before Processing: {}", rocket_fuel);

    // First concurrent access
    let fuel_length = process_fuel_borrowing_a_reference_data_race(&mut rocket_fuel);
    println!("After Processing: {}, {}", fuel_length, rocket_fuel);
    println!();

    // Second concurrent access without synchronization
    let fuel_length_2 = process_fuel_borrowing_a_reference_data_race(&mut rocket_fuel);
    println!("After Processing // Data Race: {}, {}", fuel_length_2, rocket_fuel);
}

fn process_fuel_borrowing_a_reference_data_race(fuel: &mut String) -> i32 {
    fuel.push_str(" is highly flammable");
    println!("Processing Fuel: {}", fuel);
    let length = fuel.len() as i32;
    println!("Length of Fuel: {}", length);
    length
}
