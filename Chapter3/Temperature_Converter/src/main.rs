//Convert temperatures between Fahrenheit and Celsius
use std::io;

fn convert_to_fahrenheit(celsius_degree : f32) -> f32 {
    (celsius_degree * (9.0/5.0)) + 32.0
}

fn convert_to_celsius(fahrenheit_degree: f32) -> f32 {
   (fahrenheit_degree - 32.0) * (5.0/9.0) 
}

fn main() {
    
    loop {

        let mut converter_choice = String::new();
        let mut degree = String::new();
        let mut display_value: f32;

        println!("Hello! Would you like to convert from Celsius to Fahrenheit or Fahrenheit to Celsius?");
        println!("Enter 1 for the former, 2 for the latter.");

        io::stdin()
            .read_line(&mut converter_choice)
            .expect("Failed to read line");

        let converter_choice: f32 = match converter_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Thank you! Now enter the degree.");
    
        io::stdin()
            .read_line(&mut degree)
            .expect("Failed to read line");

        let degree: f32 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if converter_choice == 1.0 {
            let display_value = convert_to_fahrenheit(degree);       
            println!("{degree} Celsius is {display_value} Fahrenheit");
        } else if converter_choice == 2.0 {
            let display_value = convert_to_celsius(degree);
            println!("{degree} Fahrenheit is {display_value} Celsius");
        }
    }
}
