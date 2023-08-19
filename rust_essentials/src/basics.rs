pub(crate) fn simulation() {
    // define a new variable 'x'
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 10;
    println!("The value of x is {}", x);

    // let mut y:u8 = 255; // unsigned values cant be negative 0..255
    // println!("The value of x is {}", y);
    //
    // // increase the value of y
    // y += 1;
    // println!("The value of y is {}", y);

    // define a new float variable 'y'
    let y = 5.123;
    println!("The value of y is {}", y);

    // multiply the value of pi by 2
    let pi = std::f64::consts::PI;
    println!("The value of pi is {}", pi);
    let two_pi = 2.0 * pi;
    println!("The value of two_pi is {}", two_pi);

    let z: f32 = 10.12345678901234567;
    println!("The value of z is {}", z);

    let a = 10;
    let b = 3.0;
    let c = a as f64 / b + 1.0;
    println!("The value of c is {0:08.2}\n a is {1}\nonce again c is {0:.2}", c,a);
    println!("{:?}",(3,4));

    // Define a binary value
    let mut value = 0b1111_0101u8;
    println!("The value of value is {}", value);
    // print the value in integer
    println!("The value of value in integer is {}", value as u32);

    value = !value;
    println!("The value of value is {}", value);

    value = value << 1; // shift left
    println!("The value of value is {}", value);

    value = value >> 1; // shift right
    println!("The value of value is {}", value);

    let v1= true;
    let v2 = false;
    println!("The value of v1 is {}", v1);
    println!("The value of v2 is {}", v2);
    println!("NOT v1 is {}",!v1);
    println!("NOT v2 is {}",!v2);

    let c =(v1 ^ v2) || panic!();
    println!("The value of c is {}", c);

    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}'; // finger symbol
    println!("The value is {}, {}, {}", letter, number,finger);
    //println!("The value of number is {}: {}", number, number+1);

}