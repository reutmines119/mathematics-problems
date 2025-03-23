// Example of a simple Rust program to calculate the factorial of a number

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number to find its factorial:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num: u32 = input.trim().parse().expect("Please enter a valid integer.");

    match n as usize {
        0 => print!("Factorial is defined for non-negative integers."),
        _ => println!("{}", factorial(num)),
    }
}

fn factorial(n: usize) -> usize {
    if n == 1 || n == 0 {
        return 1;
    }

    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }

    result
}
