use std::io;

pub fn simulation() {
    println!("Dont Transfer Ownership of Data, Let them Borrow and return back after a while");
    println!("This is like a Car Rental System, When I rent the car, I can use, but I cant modify or sell it as I am not the owner");

    //fuel_sim();
    //fuel_sim_borrow();
    //fuel_sim_borrow_data_race();
    // fuel_sim_dangling_reference();
    //borrow_slice_sim_1();
    //borrow_slice_sim_2();
    get_nth_word_sim();
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

fn fuel_sim_dangling_reference() {
    let mut rocket_fuel = String::from("RP-1");
    println!("Before Processing: {}", rocket_fuel);
    let rocket_fuel = process_fuel_avoid_dangling_reference(); // this returns a reference to rocket_fuel
    println!("After Processing: {}", rocket_fuel); // this is a reference to a String which no longer exists
}

/*
Scenario: Dangling Reference
Here, return type is '&String' --> which is a borrow values
But there is no values for it to be borrowed from
 */
// fn process_fuel_dangling_reference() -> &String {
//     let new_fuel = String::from("LPG"); // out of this function, this new_fuel goes out of scope
//     &new_fuel   // thus this return type out of this function when returned the borrowed reference, this actually referring nothing
// }

fn process_fuel_avoid_dangling_reference() -> String {
    let mut new_fuel = String::from("LPG"); // out of this function, this new_fuel goes out of scope
    new_fuel.push_str(" is highly flammable");
    new_fuel   // thus this return type out of this function when returned the borrowed reference, this actually referring nothing
}

// Scenario: What if not to borrow whole, but a slice without taking ownership of them using '&' annotation ?
// Strings are dynamically allocated on the heap at runtime, the compiler doesnt necessarily know how long will string end up being
fn borrow_slice_sim_1() {
    let message = String::from("Welcome to University of Texas at Arlington, USA");
    println!("Before Processing: {}", message);

    // slice the string by spaces
    let message_slice = &message[0..message.len() - 1]; // this returns a reference to the entire string
    println!("After Processing/Entire Reference: {}", message_slice);

    // print the length of the message
    let message_length = message.len();
    println!("Length of Message: {}", message_length);

    let last_word = &message[30..message.len()];
    println!("After Processing/Last Word: {}", last_word);

    // split the message into words
    let words: Vec<&str> = message.split_whitespace().collect();
    println!("After Processing/Words: {:?}", words);

    let mut counter=0;
    for word in  words{
        println!("Word[{}]: {}",counter, word);
        counter+=1;

    }

    let num_list = [1,2,3,4,5,6,7,8];
    println!("After Processing/Num List: {:?}", num_list);
    let inner_list = &num_list[0..3]; // creating a slice of numlist, borrowing, not transferring ownership
    println!("After Processing/Inner List: {:?}", inner_list);
}

fn borrow_slice_sim_2() {
    let message = String::from("Welcome to University of Texas at Arlington, USA");
    // slice the strings into words and get the first word only
    let words: Vec<&str> = message.split_whitespace().collect();
    let first_word = words[0];
    let first_word = get_first_word_by_bytes(&message);
    //let first_word = get_first_word(&message);
    println!("First Word: {}", first_word);

    let message_slice = &message[0..5];
    let first_word_slice = get_first_word(message_slice);
    println!("First Word with Slice input: {}", first_word_slice);
}

// &String != &str
// In the examples above, &String is used to borrow a reference to the entire String object, while &str is used to borrow a reference to a string slice. Both allow accessing and using the string data, but &String has the additional capability to modify the underlying String object,
// while &str represents an immutable view into the string data.
// message.push_str(" is highly flammable") --> is only available if &mut String; not available if &str

fn get_first_word_by_bytes(message: &String) -> &str {

    // convert the string into bytes
    let bytes = message.as_bytes(); // [87, 101, 108, 99, 111, 109, 101, 32, 116, 111, 32, 85, 110, 105, 118, 101, 114, 115, 105, 116, 121, 32, 111, 102, 32, 84, 101, 120, 97, 115, 32, 97, 116]
    println!("Bytes: {:?}", bytes);

    // iterate over to get the index and item
    for (index, item) in bytes.iter().enumerate() {
        // checked if it matches the ASCII representation of a space character (b' ').
        // If a space is found, it indicates the end of the first word.
        println!("Index: {}, Item: {}", index, item); // Index: 0, Item: 87
        if *item == b' '{
            return &message[0..index];
        }
    }
    &message // no space found, return the entire string


}

// Example: Deref Coercion
// When to use &String and &str?
fn get_first_word(message: &str) -> &str {
    // iterate over to get the index and item
    for (index, item) in message.chars().enumerate() {
        println!("Index: {}, Item: {}", index, item); // Index: 0
        if item ==' ' {
            return &message[0..index];
        }
    }
    &message
}

fn get_nth_word_sim() {
    let message = String::from("Welcome to University of Texas at Arlington, USA");
    // take a user input as integer

    let number = user_input_usize();
    let nth_word= get_nth_word(&message, number);
    println!("Nth Word: {:?}", nth_word);

}

fn user_input_usize() -> usize {
    println!("Please enter a number:");

    let mut buffer = String::new(); // stores in the heap, so it dynamically resizes
    /*
    - io::stdin() ->  from io module we use stdin 'Standard in' function to create a new handle to access the standard input stream
    - then use read_line() to read the input from the user
    - then pass a mutable reference to our buffer as an input argument to the read_line() function
     */
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    let number: usize = buffer.trim().parse().expect("Invalid input");
    number
}

fn get_nth_word(input: &str, n: usize) -> Option<&str> {
    let words: Vec<&str> = input.trim().split_whitespace().collect();
    println!("Words Vector: {:?}", words);
    if words.len() > 1 && n < words.len() {
        Some(words[n-1])
    } else {
        None
    }
}
