use std::io;
use std::cmp;
use rand::prelude::*;

pub fn challenge() {
    let mut play_again = true;
    let mut playing_term = 0;

    while play_again {
        playing_term += 1;
        println!("Welcome to the guessing game!");
        println!("Playing for the {} term", playing_term);

        let mut guess_counter = 0;
        // generates a random number between 1 and 100
        let mut number_to_guess = thread_rng().gen_range(1..=100);
        println!("Number to guess: {}", number_to_guess);

        // guess the number
        println!("Guess a number between 1 and 100");

        loop {
            let mut my_guess = String::new();
            println!("Enter your guess: ");
            io::stdin().read_line(&mut my_guess)
                .expect("Failed to read line");
            let my_guess: u32 = my_guess.trim().parse().expect("Invalid number");

            guess_counter += 1;
            println!("You have guessed {} times", guess_counter);

            // if my_guess < number_to_guess {
            //     println!("Too low!");
            // } else if my_guess > number_to_guess {
            //     println!("Too high!");
            // }  else {
            //     println!("You got it!");
            //     break;
            // }


            /*
            It uses a match expression to compare the values.
            It explicitly handles each variant of the Ordering enum.
            It may be more concise when there are more than two comparison cases.
            It is well-suited for situations where you want to handle each case explicitly and perform different actions based on the comparison result.
             */

            match my_guess.cmp(&number_to_guess) {
                cmp::Ordering::Less => println!("Too low!"),
                cmp::Ordering::Greater => println!("Too high!"),
                cmp::Ordering::Equal => {
                    println!("You got it!");
                    break;
                }
            }

            if guess_counter >= 3 {
                println!("Out of guesses! The number was: {}", number_to_guess);
                break;
            }
        } // loop ends here

        println!("Do you want to play again? (yes/no)");
        let mut response = String::new();
        io::stdin().read_line(&mut response)
            .expect("Failed to read line");

        play_again = response.trim().eq_ignore_ascii_case("yes"); // If the response is "yes" (ignoring case), play_again remains true and the game restarts.
    }
    println!("Thank you for playing!");
}