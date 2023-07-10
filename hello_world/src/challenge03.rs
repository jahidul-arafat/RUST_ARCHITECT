use std::any::type_name;

pub(crate) fn challenge() {
    // define an 1D array of 10 integers
    let numbers = [10, 20, 1, 2, 4, -30, 6, 7, 8, 9, 10];
    // determine max,min and average
    // let max = numbers.iter().max().unwrap(); // &i32 represents a reference to an integer; &i32 is a reference to the memory location where the value is stored.
    // // max() method returns an Option<&T> value, the unwrap() method is called to extract the maximum value from the Option
    // // Using references allows us to access or borrow the value without taking ownership of it.
    // // If you need to access the actual value of max (the maximum value from the numbers array), you can dereference the reference using the * operator, like this:
    // // // let actual_max: i32 = *max;
    //
    //
    // let min = numbers.iter().min().unwrap();
    // let sum = numbers.iter().sum::<i32>();
    // let avg = sum as f32 / numbers.len() as f32;
    // println!("max: {}, min: {}, sum: {}, avg: {:.2}", max, min, sum, avg);

    let mut max: i32;
    let mut min: i32;
    let mut sum: f64;
    let mut mean: f64;

    max = numbers[0];
    min = numbers[0];
    mean = 0.0;
    sum = 0.0;
    for num in numbers {
        if num > max {
            max = num;
        } else if num < min {
            min = num;
        }
        sum += num as f64;
    }
    mean = sum / numbers.len() as f64;
    println!("max: {}, min: {}, mean: {:.2}", max, min, mean);

    // let value: i32 = 42;
    // let reference_1: &i32 = &value;
    // let reference_2: &&i32 = &reference_1;
    //
    // println!("Value: {}", value);
    // println!("Reference 1: {}", *reference_1);
    // println!("Reference 2: {}", **reference_2); // * --> for dereferencing, & --> for borrowing
}