use std::io;
extern crate rand;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..11);

    let mut guess = String::new();
    let mut attempts_left = 3;
    
    println!("Guess a number between {} and {}!", 1, 10);

    loop {
        if attempts_left == 0 { break };
        
        attempts_left -= 1;

        guess.clear();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter in a number!");
                continue;
            }
        };
        
        if guess > secret_number {
            println!("Too high! Tries left: {}", attempts_left);
            continue;
        }
        if guess < secret_number {
            println!("Too low!. Tries left: {}", attempts_left);
            continue;
        }

        println!("You guessed it! The answer was {}.", secret_number);
        break;
    }

}
