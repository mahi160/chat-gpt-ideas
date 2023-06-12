/**
 * Word Counter
 * A program that reads a text file and counts the occurrences of each word. It can display the results in a sorted manner, showing the most frequent words first.
 */
use utils;
pub fn run() {
    let file_path = utils::take_input(Some("Provide a file path to read"));
    let file = utils::read_file(&file_path).expect("No File");
    let data = utils::file_to_string(file).unwrap();

    println!("{}", data);
}
