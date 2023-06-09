use std::io;
pub fn run() {
    println!("Enter temperature:");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    let temp: f64 = temp.trim().parse::<f64>().unwrap();

    println!("Enter Unit (C or F or K):");
    let mut scale = String::new();
    io::stdin()
        .read_line(&mut scale)
        .expect("Failed to read scale");
    let scale: &str = scale.trim();

    if scale == "F" {
        println!("{:.2}°F is {:.2}°C", temp, (temp - 32.0) * 5.0 / 9.0);
        println!(
            "{:.2}°F is {:.2}°K",
            temp,
            (temp - 32.0) * 5.0 / 9.0 + 273.15
        );
    }
    if scale == "C" {
        println!("{:.2}°C is {:.2}°F", temp, (temp * 9.0 / 5.0) + 32.0);
        println!("{:.2}°C is {:.2}°K", temp, temp + 273.15);
    }
    if scale == "K" {
        println!("{:.2}°K is {:.2}°C", temp, temp - 273.15);
        println!(
            "{:.2}°K is {:.2}°F",
            temp,
            (temp - 273.15) * 9.0 / 5.0 + 32.0
        );
    }

    if scale != "K" && scale != "F" && scale != "C" {
        println!("Invalid scale. Please use F, C or K.");
    }
}
