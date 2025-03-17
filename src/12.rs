fn main() {
    let mut rng = rand::thread_rng();
    let num1: i32 = rng.gen_range(1..10);
    let num2: i32 = rng.gen_range(1..10);
    let result = num1 + num2;
    println!("The sum of {} and {} is {}.", num1, num2, result);
}
