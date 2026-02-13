---
title: Simple Console Claculator in Rust
---

# Goal

In this Tutorial you will learn how to create a simple console-based calculator in Rust.
The calculator will:

- Ask the user for two numbers
- Aks for an operator(+, -, *, /)
- Calculate the result
- Handle division by zero
- Handle invalid operators

By the end, you will understand  basic Rust syntax, user input, parsing, and match statements.

# Previous Knowledge

We assume that:

- You have Rust installed
- You know how to run a Rust programm using `cargo run`
- You understand  basic programming concepts like variables andfunctions
- You have basic knowledge of datatypes

# What you'll learn

In this tutorial, you will learn:

- How to print using `println!`
- How to read user input using `std::io`
- What `use std::io;` means
- How to convert a `String` into a number using `.parse()`
- How to use the `match` statement
- How to handle simple errors
- How to prevent division by zero

# Tutorial

## Step 1 – Import the Input/Output Module

First, import Rust’s standard input/output library:

```rust
use std::io;
```

- `std` = standard library  
- `io` = input/output  

This allows us to read input from the console.

---

## Step 2 – Create the Main Function

Every Rust program starts with a `main` function:

```rust
fn main() {
```

---

## Step 3 – Print a Welcome Message

```rust
println!("Calculator");
println!("-------------");
```

`println!` prints text to the console.

---

## Step 4 – Read the First Number

```rust
println!("Enter first number:");
let mut first_input = String::new();

io::stdin()
    .read_line(&mut first_input)
    .expect("Failed to read line");

let first_number: f64 = first_input
    .trim()
    .parse()
    .expect("Please enter a valid number");
```

### Explanation

- `String::new()` creates an empty string  
- `mut` makes the variable changeable  
- `read_line()` reads user input  
- `.trim()` removes spaces and newline characters  
- `.parse()` converts the string into a number  
- `f64` is a decimal number type  
- `.expect()` shows an error message if something fails  

---

## Step 5 – Read the Operator

```rust
println!("Enter operator (+, -, *, /):");
let mut operator_input = String::new();

io::stdin()
    .read_line(&mut operator_input)
    .expect("Failed to read line");
```

---

## Step 6 – Read the Second Number

```rust
println!("Enter second number:");
let mut second_input = String::new();

io::stdin()
    .read_line(&mut second_input)
    .expect("Failed to read line");

let second_number: f64 = second_input
    .trim()
    .parse()
    .expect("Please enter a valid number");
```

---

## Step 7 – Perform the Calculation Using `match`

```rust
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
```

### Why use `match`?

`match` works like an improved `if/else` statement.  
It checks different possible values and executes the correct code block.

The `_` is the default case (if the operator is not valid).

---

## Step 8 – Print the Result

```rust
println!("Result: {}", result);
}
```

`{}` is a placeholder that gets replaced with the value of `result`.

---

# Full Code

```rust
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
```

---

# Result

Run the program with:

```
cargo run
```

Example output:

```
Calculator
-------------
Enter first number:
2
Enter operator (+, -, *, /):
*
Enter second number:
3
Result: 6
```

![Bildschirmaufnahme 2026-02-13 102750 (1)](https://github.com/user-attachments/assets/3af9cbce-ea9f-4929-bedb-99adfd33cc7e)




---

# What Could Go Wrong?

Here are some common mistakes that can happen while coding:

---

## 1. Forgetting `mut`

If you write:

```rust
let first_input = String::new();
```

instead of:

```rust
let mut first_input = String::new();
```

You will get an error like:

```
cannot borrow as mutable
```

This happens because `read_line()` needs a mutable reference (`&mut`).
If the variable is not declared as `mut`, Rust does not allow it to be changed.

---

## 2. Forgetting to Import `std::io`

If you forget this line:

```rust
use std::io;
```

You will get an error saying that `io` cannot be found.

Rust needs this import to use `stdin()`.

---

## 3. Type Mismatch Errors

If you try to calculate using `String` instead of converting to `f64`, like this:

```rust
let result = first_input + second_input;
```

It will not compile.

Rust does not allow adding strings like numbers.
You must convert them using `.parse()` first.

---

## 4. Missing `.trim()`

If you forget:

```rust
operator_input.trim()
```

and just write:

```rust
match operator_input {
```

The program may not recognize the operator correctly.

This is because `read_line()` stores a newline character (`\n`).
`.trim()` removes that invisible character.

---

# Conclusion

You have successfully created a simple console calculator in Rust 

You learned:

- Rust syntax basics  
- Input and output  
- Parsing strings into numbers  
- Using `match`  
- Basic error handling  

From here, you could improve the calculator by:

- Adding a loop so it runs multiple times  
- Creating functions  
- Using `Result` instead of `expect()`  
- Adding more mathematical operations  
