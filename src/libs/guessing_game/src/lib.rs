use rand::{self, Rng};
use std::{cmp::Ordering, io};

pub fn run() {
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);
    let input = io::stdin();
    let mut count = 0;

    println!("Guess a number between 1 and 100. You will get 5 chances!");

    loop {
        if count == 5 {
            println!("You Lose! The number is {}", random_number);
            break;
        }

        let mut guess = String::new();
        println!("Guess {}: ", count + 1);
        input.read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = guess.trim().parse().unwrap();

        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("You win & it took you {} guesses!", count + 1);
                break;
            }
            Ordering::Greater => println!("Too High!"),
            Ordering::Less => println!("Too Low!"),
        }
        count += 1;
    }
}
