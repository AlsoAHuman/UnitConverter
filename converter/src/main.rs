use std::io::{self, Write};

fn main() {
    // Unit Conversion Prompt
    println!(
        "What units would you like to convert?
    Options:
    (1) Meters to Feet
    (2) Celcius to Fahrenheit"
    );

    // Unit Selection Input
    let mut input_string = String::new();
    print!("Enter a number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let unit_selection: f64 = input_string.trim().parse().unwrap_or(0.0);
    println!();

    // Unit Selection Logic
    if unit_selection == 1.0 {
        length_conversion()
    } else if unit_selection == 2.0 {
        temperature_conversion()
    } else {
        println!("Invalid option. Please enter either 1 or 2.");
        println!();
    }
}

fn length_conversion() {
    println!("Length Test");
    print!("Enter Amount: ");

    // User Input
    let mut input_string = String::new();
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let unit_amount: f64 = input_string.trim().parse().unwrap_or(0.0);

    // Conversion Logic
    println!(
        "{} Meters is equal to {} Feet",
        unit_amount,
        unit_amount * 3.281
    );
}

fn temperature_conversion() {
    println!("Temperature Conversion");
    print!("Enter Amount: ");

    // User Input
    let mut input_string = String::new();
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let unit_amount: f64 = input_string.trim().parse().unwrap_or(0.0);

    // Conversion Logic
    println!(
        "{} Celcius is equal to {}",
        unit_amount,
        (unit_amount * 1.8) + 32.0
    );
}
