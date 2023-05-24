use rand::{self, Rng};
use std::{cmp::Ordering, io};

pub fn guessing_game() {
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);

    let input = io::stdin();
    let mut count = 0;

    println!("Guess a number between 1 and 100. You will get 5 chances!");

    // for i in 0..5 {
    //     let mut guess = String::new();
    //     println!("Guess #{}", i + 1);
    //     input.read_line(&mut guess).expect("Failed to read line");
    //     let guess: u32 = guess.trim().parse().unwrap();
    //     match guess.cmp(&random_number) {
    //         Ordering::Less => println!("Too Low!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //         Ordering::Greater => println!("Too High!"),
    //     }
    // }

    loop {
        let mut guess = String::new();
        println!("Guess: ");
        count += 1;

        input.read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = guess.trim().parse().unwrap();

        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too High!"),
            Ordering::Less => println!("Too Low!"),
        }
    }
    println!(
        "The number was {} & it took you {} guesses!",
        random_number, count
    );
}
