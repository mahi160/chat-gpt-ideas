use currency_converter;
use file_analyzer;
use guessing_game;
use std::time::Instant;
use temp_conv;
use todo_list;
use url_shortener;
use web_scrapper;
use word_count;

use utils::take_input;
fn main() {
    println!("Hello, world! These are my first rust projects! Please select one to use!");
    let choices = [
        (1, "Currency Converter"),
        (2, "File Analyzer"),
        (3, "Guessing Game"),
        (4, "Temperature Conversion"),
        (5, "Todo List"),
        (6, "URL Shortener"),
        (7, "Web Scrapper"),
        (8, "Word Count"),
    ];
    for elm in choices.iter() {
        println!("{}: {}", elm.0, elm.1);
    }

    let choice = take_input(None).trim().parse().unwrap();

    let start_time = Instant::now();
    match choice {
        1 => currency_converter::run(),
        2 => file_analyzer::run(),
        3 => guessing_game::run(),
        4 => temp_conv::run(),
        5 => todo_list::run(),
        6 => url_shortener::run(),
        7 => web_scrapper::run(),
        8 => word_count::run(),
        _ => println!("Please enter a valid number"),
    };
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    println!("Elapsed time: {:?}", elapsed_time);
}
