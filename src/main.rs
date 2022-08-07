use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a Number!\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}\n");

    loop {
        println!("Please input your guess :\n");

        // variables are immutable by default in Rust, 'mut' keyword is used to make it mutable

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.\n");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}\n");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small\n"),
            Ordering::Greater => println!("Too big\n"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        }
    }
}
