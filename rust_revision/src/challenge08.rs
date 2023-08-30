use std::ops::{Add, Mul, Sub, Div};


pub(crate) fn challenge() {
    println!("Hello, world!");

    let num1 = Box::new(10f64);
    let num2 = Box::new(20f64);
    // let output = add_two(one, two);
    // //println!("a:{}",one); // ownership already transferred to add_two() function
    // println!("Output: {}", *output + 1); // unbox, dereferencing to get the actual value
    let output = math_ops(num1, num2, "div")
        .expect("Error in math_ops; unable to perform math operation");
    println!("Output: {:?}", *output); // unboxing // dereferencing to get the actual value
}

// method to add two boxed numbers
// ownership of a and b is transferred to the function
// Adding trait Add to the Generic Type T; Because T dones know
fn add_two<T: Add<Output=T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    let sum = *a + *b; // unboxing a and b and get the values
    Box::new(sum)
}

/*
Output Result<> is an enum, which has two variants: Ok(T) and Err(E)
- Ok(T) means that the operation was successful, and T is the result of the operation
- Err(E) means that the operation failed, and E is the reason for the failure
 */
fn math_ops<T>(a: Box<T>, b: Box<T>, choice:&str) -> Result<Box<T>, &str>
    where T:
        Add<Output=T> +
        Sub<Output=T> +
        Mul<Output=T> +
        Div<Output=T> +
        PartialOrd +
        num_traits::Zero // Not an error // this Trait has a dependency. You have to add this in the Cargo.toml file
{
    match choice {
        "add" => Ok(Box::new(*a + *b)),
        "sub" => Ok(Box::new(*a - *b)),
        "mul" => Ok(Box::new(*a * *b)),
        "div" => {
            // *b -> will deference the T from the Box<T> and we dont know what T could be
            if *b == T::zero(){
                return Err("division by zero");
            }
            Ok(Box::new(*a / *b))
        },
        _ => Ok(Box::new(T::zero())),
    }
}
