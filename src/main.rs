use std::io;

fn main() {
    println!("Calculator");
    println!("-------------");

    println!("Enter first number:");
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).expect("Failed to read line");
    let first_number: f64 = first_input.trim().parse().expect("Please enter a valid number");

    println!("Enter operator (+, -, *, /):");
    let mut operator_input = String::new();
    io::stdin().read_line(&mut operator_input).expect("Failed to read line");

    println!("Enter second number:");
    let mut second_input = String::new();
    io::stdin().read_line(&mut second_input).expect("Failed to read line");
    let second_number: f64 = second_input.trim().parse().expect("Please enter a valid number");

    let result = match operator_input.trim() {
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" => first_number * second_number,
        "/" => {
            if second_number == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            } else {
                first_number / second_number
            }
        }
        _ => {
            println!("Error: Invalid operator.");
            return;
        }
    };
    println!("Result: {}", result);
}
