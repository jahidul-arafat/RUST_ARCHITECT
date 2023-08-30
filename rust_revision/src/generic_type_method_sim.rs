pub(crate) fn simulation(){
    println!("Hello, world!");
    println!("{}", get_max(10.1,20.2));
}

// method to get the biggest of two numbers
/*
Errors
- Binary operation '>' cant be used for generic type <T>, which could be anything
- Solution:
    - Give the Data Type <T> the ability for comparison i.e. >, <, >=
    - How ? Using PartialOrd trait

 */
fn get_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}