fn main() {
    let celsius = 40.0;
        println!("{}°C is {}°F", celsius, celsius_to_fahrenheit(celsius));

    let fahrenheit = 104.0;
    println!("{}°F is {}°C", fahrenheit, fahrenheit_to_celsius(fahrenheit));
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius / 5.0) * 9.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
