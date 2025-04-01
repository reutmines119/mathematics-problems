// 1. Write a function that calculates the sum of two numbers.
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// 2. Create a Rust program that reads two integers from stdin and prints their sum.
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("Sum of {} and {}: {}", &numbers[0], &numbers[1], add(*numbers[0], *numbers[1]));
}
