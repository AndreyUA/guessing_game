use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    println!("Guess the number!!!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut step_counter: i32 = 1;

    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                step_counter = step_counter + 1;
                println!("Too small!");
            }
            Ordering::Greater => {
                step_counter = step_counter + 1;
                println!("Too big!");
            }
            Ordering::Equal => {
                let duration = start_time.elapsed();

                if step_counter == 1 {
                    println!(
                        "You win the game in 1 step in {:?} seconds! Awesome!",
                        duration.as_secs()
                    );
                } else {
                    println!(
                        "You win the game in {} steps in {:?} seconds! Cool!",
                        step_counter,
                        duration.as_secs()
                    );
                }

                break;
            }
        }
    }
}
