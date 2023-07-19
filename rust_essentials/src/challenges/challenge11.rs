use std::{cmp, io};
use rand::{Rng, thread_rng};

pub fn challenge(){
    println!("Higher and Lower Guessing Game Challenge");
    higher_and_lower_guessing_game();
}

// function to implement a higher and lower guessing game
pub fn higher_and_lower_guessing_game(){

    let mut play_again = true;
    let mut playing_term = 0;

    while play_again {
        playing_term+=1;
        println!("Welcome to the guessing game!");
        println!("Playing for the {} term", playing_term);

        let mut guess_counter = 0;
        // generates a random number between 1 and 100
        let mut number_to_guess = thread_rng().gen_range(1..=100);
        println!("Number to guess: {}", number_to_guess);

        // guess the number
        println!("Guess a number between 1 and 100");
        loop {
            let mut buffer = String::new();
            // io::stdin().read_line(&mut my_guess)
            //     .expect("Failed to read line");
            // let my_guess: u32 = my_guess.trim().parse().expect("Invalid number");
            let my_guess = match io::stdin().read_line(&mut buffer) {
                Ok(n) => match buffer.trim().parse::<u32>() {
                    Ok(n) => n, // success
                    Err(e) => {
                        println!("Error: {}", e);
                        continue; // will continue if anything other than integers is entered
                    }
                }
                Err(err) => {
                    println!("Error: {}", err);
                    continue; // will continue if anything other than integers is entered // this will not add to the guess_counter
                }
            };

            guess_counter += 1;
            println!("You have guessed {} times", guess_counter);

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
        }
        println!("Do you want to play again? (yes/no)");
        let mut response = String::new();
        io::stdin().read_line(&mut response)
            .expect("Failed to read line");

        play_again = response.trim().eq_ignore_ascii_case("yes"); // If the response is "yes" (ignoring case), play_again remains true and the game restarts.
    }
    println!("Thank you for playing!");
}