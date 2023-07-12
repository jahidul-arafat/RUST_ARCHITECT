use std::io;
//use rand::random;
use rand::prelude::*;
use rand::Rng;

pub fn simulation(){
    // let mut buffer = String::new();
    // println!("Enter a number: ");
    // io::stdin().read_line(&mut buffer);
    // println!("You entered: {}", buffer);
    //
    // let mut number: i32 = buffer.trim().parse().unwrap();
    // number += 1;
    // println!("Your number is: {}", number);

    // let rand_num=random_gen();
    // println!("Your number is: {}", rand_num);

    let number: f64 = random();
    println!("Your number is: {}", number);
    random_gen();

}

fn random_gen() {
    let number= thread_rng().gen_range(1..=100);
    println!("{}",number)

}


// Crates -> collection of RUST source code files
// If you need some libraries not available in standard library, you can find them in crates.io
/*
Binary Crates -> compile to produce an executable program
Library Crates -> contain code for other programs to use
 */
