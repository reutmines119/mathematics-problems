fn main() {
    let n = 5;
    println!("The sum of the first {} natural numbers is: {}", n, calculate_sum(n));
}

fn calculate_sum(n: usize) -> usize {
    (n * (n + 1)) / 2
}
