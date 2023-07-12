pub(crate) fn function_sim(){
    println!("Function Simulation");
    method_sim(12);
    let add_value = method_add(10,20);
    println!("add_value = {}",add_value);

    let square_value = method_square(10);
    println!("square_value = {:?}",square_value); // {:?} this is a Debug formatter, {} is the default formatter
                                                    // {} default formatter cant handle tuple
}

// method with an integer parameter
// fn method_sum(x:i32) -> (){} // this returns nothing, we call it '()' unit data type;
// RUST will automatically infer the return type '()'
fn method_sim(x: i32){
    println!("Method Simulation with x = {}", x);
}
/*
let sum = a+b; // this is a statement, where 'a+b' is the expression
 */
// method to add two integer numbers
fn method_add(x: i32, y: i32) -> i32{
    x + y // this is an Expression, not a Statement. Expression in RUST dont have a semicolon
}

// method to square an integer number and return a tuple of two integer numbers
fn method_square(x: i32) -> (i32,i32){
    (x,x*x)
}