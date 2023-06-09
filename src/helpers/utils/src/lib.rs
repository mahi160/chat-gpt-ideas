use std::io;
pub fn take_input(text: Option<&str>) -> String {
    let mut var = String::new();
    if text.is_some() {
        println!("{:?}", text);
    }
    io::stdin()
        .read_line(&mut var)
        .expect("Could not read data.");
    var
}
