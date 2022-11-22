use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!!");

    let secret_number = rand::thread_rng().gen_range(1..10);

    loop {
        println!("Please input your number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = guess.trim().parse().expect("Please parse a number");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Guessed right");
                break;
            }
            Ordering::Less => println!("Guess is lower"),
            Ordering::Greater => println!("Guess is higher"),
        }
    }
    println!("The secret number is: {secret_number}");
}
