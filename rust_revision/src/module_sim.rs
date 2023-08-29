use std::io;
//use rand::random;
//use rand;
use rand::prelude::*;

// create -> means collection of rust source code files
// Binary Crates: all of the programs are technically binary crates with a main.rs source file
// Library Crates: 3rd party library crates
// check crates.io for more info
pub(crate) fn simulation() {
    println!("Module Simulation");
    // let input1 = take_user_input();
    // let input2 = take_user_input();
    // println!("output: {}", input1 + input2);

    let mut rand_number = generate_random_number();
    println!("random number (without seed): {}", rand_number);

    rand_number = generate_random_number_with_seed();
    println!("random number (with seed): {}", rand_number);
}

fn take_user_input() -> f64 {
    // define a string buffer
    let mut input = String::new(); // string stores the data in Heap which has unrestricted amount of spaces

    println!("Enter a message: ");

    // read the input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read input"); // storing data at Heap

    // parse the input
    //let input: Vec<&str> = input.split_whitespace().collect();

    // parse the input as integer
    let input: f64 = input.trim().parse().unwrap(); // trim to get rid of newline character at the end

    // return the parsed input
    input

    //input.to_string()
}

fn generate_random_number() -> f64 {
    //let rand_number = rand::random::<f64>();

    // generate a random number between 1 to 100
    let rand_number: f64 = thread_rng().gen_range(1..101) as f64;

    // convert the random number to a float
    //println!("random number: {}", rand_number);

    rand_number
}

// method to generate a random number between 1 to 100 with seeding
fn generate_random_number_with_seed() -> f64 {
    // Define the seed value (you can change this to any value you want)
    // Define the seed value (u64)
    let seed: u64 = 1234567891; // Change this to any u64 value you wan

    let mut rng = StdRng::seed_from_u64(seed);
    let rand_number = rng.gen_range(1..101) as f64;
    rand_number
}