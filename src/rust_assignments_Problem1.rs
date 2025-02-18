const FREEZING_POINT_F: f64 = 32.0;


pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}


pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}


pub fn run() {
    let mut temp_f: f64 = 32.0;

    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{:.2}°F is {:.2}°C", temp_f, temp_c);

    for _ in 1..=5 {
        temp_f += 1.0;
        let converted_c = fahrenheit_to_celsius(temp_f);
        let converted_f = celsius_to_fahrenheit(converted_c); // ✅ Now `celsius_to_fahrenheit` is used

        println!(
            "{:.2}°F is {:.2}°C, {:.2}°C is {:.2}°F",
            temp_f, converted_c, converted_c, converted_f
        );
    }
}
