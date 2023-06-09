use rand::{self, Rng};
use std::{cmp::Ordering, io};

pub fn guessing_game() {
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);
    let input = io::stdin();
    let mut count = 0;

    println!("Guess a number between 1 and 100. You will get 5 chances!");

    loop {
        let mut guess = String::new();
        println!("Guess: ");

        input.read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = guess.trim().parse().unwrap();

        if count > 5 {
            println!("You Lose! The number is {}", random_number);
            break;
        }
        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too High!"),
            Ordering::Less => println!("Too Low!"),
        }
        count += 1;
    }
    println!(
        "The number was {} & it took you {} guesses!",
        random_number, count
    );
}
