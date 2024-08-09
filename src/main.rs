use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!(">>>>>>>>> Guess the number <<<<<<<<");

    loop {
        play();
        print!("Do you want to play again? (y/n): ");
        std::io::stdout().flush().unwrap();

        let mut play_again = String::new();

        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.trim() != "y" {
            break;
        }
    }

    println!("Goodbye!");
}

fn play() {
    let num = rand::thread_rng().gen_range(0..11);

    print!("Please input your guess: ");
    std::io::stdout().flush().unwrap();

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);
    println!("The number is: {}", num);

    if guess == num {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}
