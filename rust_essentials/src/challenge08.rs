use std::ops::{Add, Div, Mul, Sub};

// function to add two numbers stored within Box<T> object
fn ops_two_numbers<T>(a: Box<T>, b: Box<T>) -> Box<T>
where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
{
    //Box::new(*a + *b)
    //Box::new(*a - *b)
    //Box::new(*a * *b)
    Box::new(*a / *b)
}

// function to compare two numbers stored within Box<T> object
fn compare_two_numbers<T: PartialEq>(a: Box<T>, b: Box<T>) -> bool {
    *a == *b
}
// function to multiply two numbers stored within Box<T> object
fn multiply_two_numbers<T: Mul<Output=T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    Box::new(*a * *b)
}

pub fn challenge() {
    println!("Challenge 08/Generic Type/Struct/Function/Box/Dereference Operator");
    let a = Box::new(5);
    let b = Box::new(10);
    let c = ops_two_numbers(a, b); // once the addition operaiton is done, a and b no longer owns their values, the ownership transfers to c
    //let d = add_two_numbers(a,b);
    let d = Box::new(20);
    // assert_eq!(*d, 20);
    // assert_eq!(*c, 15);
    // assert_eq!(*a, 5);
    // assert_eq!(*b, 10);
    println!("a + b = {}", c);
    let e = ops_two_numbers(c, d);
    //assert_eq!(*e, 35);
    println!("c + d = {}", e);
    //println!("c = {}, d = {}", c, d);

    // print amount of memory used by c
    //println!("Amount of memory used by c(HEAP): {}", std::mem::size_of_val(&c));
    //println!("Amount of memory used by a(HEAP): {}", std::mem::size_of_val(&a));
    //println!("Amount of memory used by b(HEAP): {}", std::mem::size_of_val(&b));
    println!();
    println!("Amount of memory used by e(STACK): {}", std::mem::size_of_val(&e));

    //
    let f1 = Box::new(5.0);
    let f2 = Box::new(10.1);
    let f3 = ops_two_numbers(f1, f2);
    //assert_eq!(*f3, 15.1);
    println!("f1 + f2 = {}", f3);
    //println!("f1 = {}, f2 = {}, f3={}", f1, f2, f3);
    println!("Amount of memory used by f3(HEAP): {}", std::mem::size_of_val(&f3));

    let g1 = Box::new(5.0);
    let g2 = Box::new(10.1);
    let g3 = compare_two_numbers(g1, g2);
    println!("g1 == g2 = {}", g3);
    assert_eq!(g3, false);
}

