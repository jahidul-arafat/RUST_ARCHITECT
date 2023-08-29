use std::env; // std::env::args -> to read command line arguments

pub(crate) fn simulation() {
    println!("Argument CLI Operations");
    passing_arguments_at_commandline();
}

fn passing_arguments_at_commandline() {
    if env::args().len() < 4 {
        println!("Not enough arguments");
        return;
    }
    // collect the arguments into a vector
    let args: Vec<String> = env::args().collect(); //consider giving `args` an explicit type
    println!("Arguments Vector: {:?} and Length: {}", args,args.len());
    println!("First argument: {:?}", args[1]);

    // iterating over the arguments // check if you transferred the ownership or let the loop borrow
    for arg in args.iter()  {
        println!("Argument: {:?}", arg.to_lowercase());
    }

    println!("Arguments Vector: {:?} and Length: {}", args,args.len());


    // iterating over the arguments
    // for (index, arg) in env::args().enumerate() {
    //     println!("Argument {}: {}", index, arg);
    // }

    // // get the second command line argument
    // let mut second_arg = env::args().nth(1).unwrap(); // nth(1)-> Second Command Line Argument // .unwrap() is used to check if the argument is present or not
    // println!("Second argument: {}", second_arg);
    // // convert the string into an integer
    // // let second_arg: i32 = second_arg.parse().unwrap();
    // // println!("Second argument: {}", second_arg+1);
    //
    // // get the third command line argument
    // let third_arg = env::args().nth(2).unwrap();
    // println!("Third argument: {}", third_arg);
}