use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
    .expect("Failed to flush stdout");
stdin().read_line(input)
.expect("Failed to read input");
}



fn main() {
    println!("Calculator");
    println!("-------------");

    let mut number1 = String::new();
    let mut number2 = String::new();
    let mut operator = String::new();

    print!("What is the first number?  ");
    read(&mut number1);

    print!("What is the second number?  ");
    read(&mut number2);

    print!("What is the operator?  ");
    read(&mut operator);
}
