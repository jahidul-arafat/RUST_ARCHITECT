pub(crate) fn simulation(){
    println!("Function Simulation");
    println!("Addition of two numbers: {}", add(1, 2));

    let (x,x_sq)=square(4);
    println!("Square of {} is {}", x,x_sq);
}

fn add(x: i32, y: i32) -> i32 {
    x+y // this is an expression, not a statement. That's why not needed any semicolon
}

fn square(x: i32) -> (i32,i32) {
    (x,x*x)

}