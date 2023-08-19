pub(crate) fn simulation() {
    println!("Basic Rust Simulation By Jahidul Arafat");

    let x = 10;
    let y = 20;
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The sum of x and y is {}", x + y);
    println!("Updated X:{}, Y:{}", x + 1, y + 1);

    let z = 1.234;
    println!("The value of z is {}", z);

    // define PI
    let pi = std::f64::consts::PI;
    println!("The value of PI is {}", pi);
    println!("The value of 2PI is {}", 2.0 * pi);

    let a = 10;
    let b = 3.4;
    let c = a as f64 + b;
    println!("The value of c is {}", c);

    let d = 2^3; // bitwise xor operation
    println!("The value of d is {}", d);

    let e = 2_i32.pow(2);
    println!("The value of e is {}", e);

    let i = -100;
    println!("The value of i is {}", i);

    // define a binary value
    let binary_value = 0b1111_1010u8;
    println!("The value of binary_value is {}", binary_value as u32);


}