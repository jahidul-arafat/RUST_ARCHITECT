use std::cmp::Ordering;
use std::io;
use rand::prelude::*;

pub(crate) fn challenge() {

    let mut play_again=true;
    while play_again {
        play_the_game();
        play_again = play_again_constraint();
    }

}

fn play_again_constraint() -> bool {
    let mut play_again = String::new();
    println!("Would you like to play again? (y) >> ");
    io::stdin().read_line(&mut play_again).unwrap();
    let bool_decision= match play_again.trim().to_lowercase().as_ref() {
        "y" | "yes" => true,
        _ => false,
    };
    bool_decision

}

fn play_the_game(){
    println!("Guessing Program Simulation!");
    // hyper parameters
    let mut guess_count = 0;

    // let system generate a random number between 1 and 100
    let secret_number = generate_random_number();
    println!("Secret Number: {:?}", secret_number);

    loop {
        let mut my_guess = take_input();
        guess_count += 1;
        match my_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
        if guess_count >= 3 {
            println!("You Lose!");
            break;
        }
    }
}

// method to generate a random number between 1 and 100
fn generate_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..101);
    random_number
}

// // method to take input from user
fn take_input() -> i32 {
    // define a string buffer
    let mut input = String::new();

    println!("Guess a number between 1 to 100");

    // read input from user
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // convert input to integer
    let input: i32 = input.trim().parse().expect("Failed to parse the guess");

    // return input
    input
}




