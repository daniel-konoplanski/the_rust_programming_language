use std::{cmp::Ordering, io};
use rand::Rng;

fn main()
{
    println!("Please enter a guess:");

    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1..=100);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please provide a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("You guessed: {}", guess);
}
