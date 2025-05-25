use std::io;

fn main() {
    println!("Enter two numbers:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num1: f64 = match f64::from_str(input.trim()) {
        Ok(num) => num,
        Err(_) => panic!("Invalid number format"),
    };
    println!("Enter the operation (+, -, *, /):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operator: char = match input.trim().chars().next() {
        Some(c) => c as char,
        None => panic!("Invalid input"),
    };

    let num2: f64 = match f64::from_str(input.trim()) {
        Ok(num) => num,
        Err(_) => panic!("Invalid number format"),
    };
    
    match operator {
        '+' => println!("{}", num1 + num2),
        '-' => println!("{}", num1 - num2),
        '*' => println!("{}", num1 * num2),
        '/' => {
            if num2 != 0.0 {
                println!("{}", (num1 / num2).round() as f64);
            } else {
                eprintln!("Division by zero is not allowed.");
            }
        },
    }
}
