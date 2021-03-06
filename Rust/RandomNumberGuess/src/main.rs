use rand::prelude::*;
use std::io::{self, Write};
fn main() {
    let secret = rand::thread_rng().gen_range(1, 10_u32);
    let stdin = io::stdin();

    let tries = 3;
    println!("You have {} tries to guess the number. Good luck!", tries);
    for _ in 0..tries {
        // Note that stdout is frequently line-buffered by default so it may be necessary
        // to use io::stdout().flush() to ensure the output is emitted immediately.
        print!("Your guess: ");
        let _ = io::stdout().flush();

        let mut line = String::new();
        let _ = stdin.read_line(&mut line);

        // No error handling - panic if parsing fails
        let guess: u32 =
            line
                .trim()
                .parse()
                .unwrap();

        if secret < guess {
            println!("I am less than that");
        } else if secret > guess {
            println!("I am greater than that");
        } else {
            println!("Congratulations, you won!");
            return;
        }
    }
    println!("The number was {}", secret);
}
