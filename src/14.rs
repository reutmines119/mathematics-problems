fn main() {
    let mut rng = rand::thread_rng();
    let n1: i32 = rng.gen_range(0..100);
    let n2: i32 = rng.gen_range(0..100);
    println!("The sum of {} and {} is {}", n1, n2, n1 + n2);
}
