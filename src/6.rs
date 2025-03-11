  pub fn problem_2() -> u32 {
      let mut x = 1;
      while x < 100 {
          if x % 3 == 0 || x % 5 == 0 {
              println!("FizzBuzz");
          } else if x % 3 == 0 {
              println!("Fizz");
          } else if x % 5 == 0 {
              println!("Buzz");
          } else {
              println!("{}", x);
          }
          x += 1;
      }
      
      return x;
  }