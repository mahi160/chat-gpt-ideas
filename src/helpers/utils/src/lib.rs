use std::io;
pub fn take_input(text: &str) -> String {
    let mut var = String::new();
    println!("{}", text);
    io::stdin()
        .read_line(&mut var)
        .expect("Could not read data.");
    var
}
