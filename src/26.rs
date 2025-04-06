// This is a simple example demonstrating Rust's support for closures. The `addition` function takes two integers as arguments,
// adds them together, and returns their sum.
fn addition(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    // Example usage of the `addition` function
    let result = addition(5, 7);
    println!("The sum is: {}", result);
}
