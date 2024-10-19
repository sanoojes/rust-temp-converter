use std::io::stdin;

use temprature_converter::temperature::{celsius, fahrenheit, kelvin};

fn main() {
    let mut times: i32 = 0;

    loop {
        times += 1;

        if times != 3 {
            let user_input: char = get_user_input("What do you want to do ? \n1. Convert To Celsius\n2. Convert To fahrenheit\n3. Convert To Kelvin").trim().chars().nth(0).unwrap_or(' ');

            match user_input {
                '1' => {
                    let user_input = get_user_input("Do you want to convert it from Fahrenheit or Kelvin?\n1.Kelvin\n2.Fahrenheit").trim().chars().nth(0).unwrap_or(' ');

                    match user_input {
                        '1' => {
                            let user_input: f32 = get_user_input("Enter Kelvin Value:")
                                .trim()
                                .parse()
                                .unwrap();

                            let converted_value: f32 = celsius::convert_from_kelvin(user_input);
                            println!("Converted value to Celsius: {converted_value}°C")
                        }
                        '2' => {
                            let user_input: f32 = get_user_input("Enter Fahrenheit Value:")
                                .trim()
                                .parse()
                                .unwrap();

                            let converted_value: f32 = celsius::convert_from_fahrenheit(user_input);
                            println!("Converted value to Celsius: {converted_value}°C")
                        }

                        _ => {
                            println!("Invalid Choice Please try again.");
                        }
                    }
                }
                '2' => {
                    let user_input = get_user_input(
                        "Do you want to convert it from Celsius or Kelvin?\n1.Celsius\n2.Kelvin",
                    )
                    .trim()
                    .chars()
                    .nth(0)
                    .unwrap_or(' ');

                    match user_input {
                        '1' => {
                            let user_input: f32 = get_user_input("Enter Celsius Value:")
                                .trim()
                                .parse()
                                .unwrap();

                            let converted_value: f32 = fahrenheit::convert_from_celsius(user_input);
                            println!("Converted value to Fahrenheit: {converted_value}°F")
                        }
                        '2' => {
                            let user_input: f32 = get_user_input("Enter Kelvin Value:")
                                .trim()
                                .parse()
                                .unwrap();

                            let converted_value: f32 = fahrenheit::convert_from_kelvin(user_input);
                            println!("Converted value to Fahrenheit: {converted_value}F")
                        }

                        _ => {
                            println!("Invalid Choice Please try again.");
                        }
                    }
                }
                '3' => {
                    let user_input = get_user_input("Do you want to convert it from Celsius or Fahrenheit?\n1.Celsius\n2.Fahrenheit").trim().chars().nth(0).unwrap_or(' ');

                    match user_input {
                        '1' => {
                            let user_input: f32 = get_user_input("Enter Celsius Value:")
                                .trim()
                                .parse()
                                .unwrap();

                            let converted_value: f32 = kelvin::convert_from_celsius(user_input);
                            println!("Converted value to Kelvin: {converted_value}K")
                        }
                        '2' => {
                            let user_input: f32 = get_user_input("Enter Fahrenheit Value:")
                                .trim()
                                .parse()
                                .unwrap();

                            let converted_value: f32 = kelvin::convert_from_fahrenheit(user_input);
                            println!("Converted value to Kelvin: {converted_value}K")
                        }

                        _ => {
                            println!("Invalid Choice Please try again.");
                        }
                    }
                }
                _ => todo!(),
            }
        }
    }
}

fn get_user_input(msg: &str) -> String {
    let mut user_input: String = String::from("");

    println!("{msg}");

    stdin()
        .read_line(&mut user_input)
        .expect("Error when reading data.");

    user_input
}
