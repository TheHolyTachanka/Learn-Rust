extern crate rand;

use rand::thread_rng;
use rand::Rng;

fn main() {
    let secret_number = thread_rng().gen_range(1, 10);
    println!("Guess the number!");

    loop {
        println!("Please input your guess.(1-10)");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}