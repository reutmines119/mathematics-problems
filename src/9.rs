use std::num::NonZeroU32;

fn main() {
    let x = NonZeroU32::new(10).unwrap();
    println!("{}", x);
}
