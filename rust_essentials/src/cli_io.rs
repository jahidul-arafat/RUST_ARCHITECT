use std::env;

pub fn simulation(){
    println!("CLI IO Operations");
    // > cargo run Bangladesh 1971 --flag
    /*
    Output:
    Argument 0: /Users/jarotball/study/RUST_ARCHITECT/rust_essentials/target/debug/rust_essentials
    Argument 1: Bangladesh
    Argument 2: 1971
    Argument 3: --flag // to represent a configuration settings
     */

    // make sure there are at least 4 arguments
    if env::args().len() < 4 {
        println!("Please provide at least 4 arguments");
        return;
    }
    // take a command line argument and iterate over its enumerations
    for (index, argument) in env::args().enumerate(){
        println!("Argument {}: {}", index, argument);
    }

    // collect the arguments into a vector
    let arguments: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", arguments);

    // get the second command line argument
    let country = env::args().nth(1).unwrap(); // second command line argument // .unwrap() is used to check if the argument is present or not to extract the result
    let year = env::args().nth(2).unwrap(); // third command line argument
    let flag = env::args().nth(3); // fourth command line argument
    println!("Country: {}, Year: {}, Flag: {:?}", country, year, flag);

    // what happens if the argument is not present?
    // this might produce a runtime error
}

// method to take a command line argument and iterate over its enumerations and print them out
pub fn take_command_line_arguments(){
    for (index, argument) in env::args().enumerate(){
        println!("Argument {}: {}", index, argument);
    }
}
