use std::any;
use std::fmt;

pub fn simulation() {
    println!("Trait Bound Simulation");
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    print_type(String::from("thirteen"));
    print_type(Box::new(13));
    print_type([13, 1]); // throw Error, because array doesn't implement Display trait, it requires debug trait

    // create an array of integers
    let arr: [i32; 3] = [1, 2, 3];
    print_type(arr[0]);

    compare_and_print(10.0,20); // 10.0, 20.0
    compare_and_print(10.0,10.11); // 10.0, 10.0
    //compare_and_print(10,10.11); // XXXX 10, 10 (not possible)// will throw error, its feasible to convert an integer to a float, but not a float to an integer
    //compare_and_print(1,"one"); // not feasible, incompatible data types
    compare_and_print("one","two");

    // trait bound on return type
    println!("Output: {:?}", trait_bound_on_return_type(String::from("hello")));
    println!("Output: {:?}", trait_bound_on_return_type([10]));
    println!("Output: {:?}", trait_bound_on_return_type(10));


}

fn print_type<T>(item: T)
    where T: // defining the Trait Bounds
        fmt::Debug
{
    println!("{:?} is {}", item, any::type_name::<T>());
}

/*
 */
fn compare_and_print<T,U>(a: T, b: U)
    where T:
            fmt::Debug +
            PartialEq +
            From<U>, // PartialEq trait compare two values of the same datatype // but we need to compare two values of different datatypes
          U:
            fmt::Debug +
            PartialEq +
            Copy // as b is going to be converted into a later, it needs the Copy trait, to avoid transfer of ownership
{

    println!("{:?}(type {}) and {:?}(type {})", a, any::type_name::<T>(), b,any::type_name::<U>());
    // try to convert b as type of a // for that it requires another trait called 'From<U>', means converting U type data to T type data
    if a == T::from(b) { // T::from(b) this moves the ownership of b
        println!("{:?} and {:?} are equal", a, b); // b is moved and unavailable // solu: Use 'copy' trait
    } else {
        println!("{:?} and {:?} are not equal", a, b);
    }
}

// trait bound on the return type of a function
// here, the return type is trait bounded on Display trait
fn trait_bound_on_return_type<T>(input:T) -> impl fmt::Debug
    where T: fmt::Debug
{
    // if any::TypeId::of::<T>() == any::TypeId::of::<bool>() {
    //     return ("Its a boolean".to_string(),input);
    // }
    input
}

// what if the datatype and trait bounds cant be known into Runtime? Solution: Dynamic Dispatch
// fn datatype_traitbound_failing_in_runtime(choice: bool)-> impl fmt::Display {
//     // would fail, because if and else have incompatible types
//     if choice {
//         13
//     } else {
//         "thirteen" //fail
//         // 14 // pass
//     }
// }