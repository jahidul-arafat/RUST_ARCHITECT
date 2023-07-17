use std::any;
use std::fmt;
use std::fmt::{Debug, Display};

// create a function named print_type with generic type T and parameter item
fn print_type<T: Debug>(item: T) { // Generic Type T has to implement Display trait; Better if Generic Type T implements Debug trait
    println!("{:?} is {}", item, any::type_name::<T>());
}

// create a function named compare_and_print with generic type T and U
//fn compare_and_print<T: Debug + PartialEq + From<U>, U: Debug + PartialEq + Copy>(item: T, other: U) {
fn compare_and_print<T, U>(item: T, other: U)
    where T: Debug + PartialEq + From<U>,
          U: Debug + PartialEq + Copy{
    // here trait From<U> makes sure it could convert datatype U to datatype T
    // but when we use from(other), it consumes the variable 'other', effectively moving it to a new variable, which is not what we want
    // that's why we implement Copy trait for U
    if item == T::from(other) { // what if item and other are of different datatype
        // converting 'other' to Datatype T
        // this require PartialEq trait // array requires a Debug trait // else Display trait would be enough
        // PartialEq trait compare two values of same datatype; but here 'item' and 'other' are of different datatypes
        println!("{:?} and {:?} are equal", item, other);
    } else {
        println!("{:?} and {:?} are not equal", item, other);
    }
}


// Easy approach to pass all the different datatype comparison
// fn compare_and_print<T: Debug, U: Debug>(item: T, other: U) { // comparing the string representation of the two values
//     if format!("{:?}", item) == format!("{:?}", other) {
//         println!("{:?} and {:?} are equal", item, other);
//     } else {
//         println!("{:?} and {:?} are not equal", item, other);
//     }
// }

// create a function named 'get_displayable' that takes no argument and returns a Display trait implementation
fn get_displayable() -> impl Display {
    // Trait Display is not implemented for type 'array'
    // 13 // pass
    // 13.2 // pass
    String::from("Hello, world!") //pass
    //[1,2,3,4,5] // fail
}

// would fail at runtime
// error[E0308]: `if` and `else` have incompatible types
// fn get_displayable_fail(choice:bool) -> impl Display {  // RUST compiler doesn't allow this type of ambiguity in return type
// // solution is : Dynamic Dispatch
//     if choice {
//         String::from("Thirteen")
//     } else {
//         13
//     }
// }

// create a function named 'get_displayable_pass' that takes Generic datatype T and returns a Display trait implementation
fn get_displayable_pass<T: Display>(item: T) -> T {
    item
}




pub fn simulation() {
    // create an array of 10 integers
    let arr = [1, 2, 3, 4, 5];
    // print the array
    println!("{:?}", arr);
    print_type(13);
    print_type("hello");
    print_type(13.2);
    print_type(true);
    print_type(arr); // array doesnt implement Display trait, instead it implements Debug trait
    //compare_and_print(13, 13.2); // will fail, because 13.2 f64 cant be converted to i32 13
    compare_and_print(13.2, 13.2);
    compare_and_print(true, false);
    compare_and_print(13.2, 13);
    //compare_and_print(1, "one"); // failed // because string "one"" cannot be converted to integer
    //compare_and_print(13.2,arr);
    //compare_and_print([1, 2, 3, 4, 5,0],arr);

    println!("output is {}",get_displayable());
    //println!("output is {}",get_displayable_fail(true));
    println!("output is {}",get_displayable_pass(13));
    println!("output is {}",get_displayable_pass(13.2));
    println!("output is {}",get_displayable_pass("one"));
    //println!("output is {:?}",get_displayable_pass(arr)); // would fail
}