use std::io;
use std::cmp::Ordering;
use rand::{Rng, thread_rng};

fn main() {
  println!("Guess the number");

  let mut rng = thread_rng();
  let secret_number = rng.gen_range(1..=100);

  loop {
    println!("Please, input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess:u32 = match guess.trim().parse() {
      Ok(number) => number,
      Err(_) => continue,
    };

    println!("You guessed, buddy: {guess}");

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small."),
      Ordering::Greater => println!("Too big."),
      Ordering::Equal => {
        println!("You win!");
        break;
      },
    }
  }
}
