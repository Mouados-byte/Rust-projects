use std::io;

fn main() {
    let choice = get_user_choice();
    let temperature = get_temperature();

    match choice {
        1 => {
            let fahrenheit = celsius_to_fahrenheit(temperature);
            println!("{}", fahrenheit)
        },
        2 => {
            let celsius = fahrenheit_to_celsius(temperature);
            println!("{}", celsius)
        },
        _ => println!("Invalid choice")
    }
}

fn get_user_choice() -> u8 {
    println!("Enter your choice: ");
    println!("1 / celsius to fahrenheit");
    println!("2 / fahrenheit to celsius");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    choice.trim().parse().unwrap_or(0)
}

fn get_temperature() -> f64 {
    let mut temperature = String::new();

    io::stdin()
      .read_line(&mut temperature)
      .expect("failed to read input");

    temperature.trim().parse().unwrap_or(0.0)
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}