pub fn simulation(){
    println!("RUST Variable Lifetime Simulation/Example of Borrow Checker");
    let propellant; // i.e. lifetime 'a
    // let rp1=String::from("Propellant 1"); // solution-01
    {
        // inner sub scope of propellant // i.e. lifetime 'b
        let rp1=String::from("Propellant 1");
        // let propellant borrow from rp1
        propellant=&rp1; // borrowing rp1 // not transferring the ownership of rp1
        println!("Propellant: {}", propellant);
        println!("RP-1: {}", rp1); // if propellant got the ownership of rp1, then rp1 will be dropped and will no longer be available
    }
    //println!("Propellant: {}", propellant); // this is an Example of Dangling Reference
                                            // Hence the rp1 value is borrowed, this doesnt live long enough outside the scope

    let propellant2=String::from("RP-2");
    let propellant3=String::from("LPG");
    let mut result = best_fuel(&propellant2,&propellant3);
    // let result = best_fuel(propellant2,propellant3); // if not borrowing, instead passing the ownership of propellant2 and propellant3
    println!("Propellant2: {}", propellant2); // if not borrowing, then propellant2 will be dropped and will no longer be available
    println!("Propellant3: {}", propellant3); // if not borrowing, then propellant3 will be dropped and will no longer be available
    println!("Best Fuel: {}", result);

    /*
    // Failed Example
    // lets create two new propellants in different scopes
    let propellant4=String::from("LIQ"); // in lifetime 'a
    {
        let propellant5=String::from("LIQ-1"); // in lifetime 'b
        result = best_fuel(&propellant4,&propellant5); // the result gonna use the lifetime of propellant5
    }
    println!("Result: {}", result); // the result using the lifetime of propellant5 (as this is more restrictive)
                                    // due to two different lifetime used for propellant4 and propellant5
                                    // propellant5 will become out of scope and will no longer be available

    // Question: Why not compiler analyzing the parameters lifetime on its own?
    /*
    Answer:
    if the 'if else' block in the best_fuel function is from an external compiled library,
    the compiler might not have access to that source code and analyze what its doing.
    That's why compiler doesnt determine the parameters lifetime on its own.
    That's why we need to explicitly annotate the lifetime of the parameters.
     */

     */

    let message= String::from("Hello, World!");
    let first_word = get_first_word(&message);
    println!("Message: {}", message);
    println!("First Word: {}", first_word);

} // main simulation function ends here

//fn best_fuel(x:String, y:String) -> String {
//fn best_fuel(x:&str, y:&str) -> &str {
//fn best_fuel<'a, 'b>(x:&'a str, y:&'b str) -> &'a str{ // two different lifetime used for x and y, and the lifetime of returned reference doesnt depend on y
fn best_fuel<'a, 'b>(x:&'a str, y:&'a str) -> &'a str{ // introducing Generic Lifetime for the input and output parameters
    // called lifetime annotation i.e. 'a
// see, we are returning a borrowed reference, but not possible to know which borrow value that would be x or y, that depends on which string is longer
    // and that cant be known until the runtime
    // Solution: is to use named lifetime parameter i.e. 'a and then provide an example function signature
    if x.len() > y.len() {
        x
    } else {
        y
        //x // use if two different lifetime annotations are used for x and y or no lifetime annotation is used for y at all
    }
}

// function to get the first word of a string slice
// see we are not explicitly specifying the lifetime of the referenced parameter
// but back in version before RUST 1.0, we had to explicitly specify the lifetime of the referenced parameters
/*
Lifetime Elision Rules - not for you, but for compiler to follow
1. Non reference parameters doesnt require a lifetime annotation



 */
// fn get_first_word<'a>(s:&'a str) -> &'a str {
fn get_first_word(s:&str) -> &str {
    let bytes = s.as_bytes();
    println!("Bytes: {:?}", bytes); // Bytes: [72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33]
    // check if bytes are not empty
    for (i, &item) in bytes.iter().enumerate() {
        // print the index and the value of the byte
        println!("{}: {}", i, item);
        if item == b' '{
            return &s[..i]; // found a space
        }
    }
    &s // if no space found, return the whole string
}