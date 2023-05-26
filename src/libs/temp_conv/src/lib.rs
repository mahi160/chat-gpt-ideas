pub fn temp_conv(temp: f64, scale: char) {
    let result: f64;
    if scale == 'F' {
        println!("{:.2}°F is {:.2}°C", temp, (temp - 32.0) * 5.0 / 9.0);
        println!("{:.2}°F is {:.2}°K", temp, (temp - 32.0) * 5.0 / 9.0 + 273.15);
    }
    if scale == 'C' {
        println!("{:.2}°C is {:.2}°F", temp, (temp * 9.0 / 5.0) + 32.0);
        println!("{:.2}°C is {:.2}°K", temp, temp + 273.15);
    }
    if scale == 'K' {
        println!("{:.2}°K is {:.2}°C", temp, temp - 273.15);
        println!("{:.2}°K is {:.2}°F", temp, (temp - 273.15) * 9.0 / 5.0 + 32.0);
    }

    if scale != 'F' && scale != 'C' && scale != 'K' {
        println!("Invalid scale. Please use F, C or K.");
    }
}
