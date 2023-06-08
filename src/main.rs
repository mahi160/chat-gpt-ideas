#![allow(unused)]
use currency_converter;
use guessing_game::guessing_game;
use std::io;
use temp_conv::temp_conv;
fn main() {
    println!("Hello, world! These are my first rust projects! Please select one to use!");
    let choices = [
        (1, "Guessing Game"),
        (2, "Temperature Conversion"),
        (3, "Currency Converter"),
    ];
    for elm in choices.iter() {
        println!("{}: {}", elm.0, elm.1);
    }

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Please enter a valid number");
    let choice: i32 = choice.trim().parse::<i32>().unwrap();
    match choice {
        1 => guessing_game(),
        2 => temp_conv(),
        3 => currency_converter::run(),
        _ => println!("Please enter a valid number"),
    };
}
