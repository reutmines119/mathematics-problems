fn main() {
    let n = 5;
    println!("The sum of the first {} natural numbers is: {}", n, sum(n));
}

fn sum(n: isize) -> isize {
    (1..=n).sum()
}
