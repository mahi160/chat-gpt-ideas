use std::collections::HashMap;
/**
 * Word Counter
 * A program that reads a text file and counts the occurrences of each word. It can display the results in a sorted manner, showing the most frequent words first.
 */
use utils;
pub fn run() {
    let file_path = utils::take_input(Some("Provide a file path to read."));
    let file = utils::read_file(&file_path.trim()).expect("No file found.");
    let data = utils::file_to_string(file).unwrap();

    let words: Vec<&str> = data.split(" ").collect();
    let mut word_count: HashMap<String, u32> = HashMap::new();

    for word in words {
        utils::insert_or_increment(&mut word_count, String::from(word))
    }

    println!("\nTop 5 words are: ",);
    for (i, word) in utils::sort_hash_by_value(&word_count).iter().enumerate() {
        if i > 5 {
            break;
        }
        println!("{} ({} times)", word.0, word.1);
    }
}
