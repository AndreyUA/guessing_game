use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
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
                if step_counter == 1 {
                    println!("You win the game in 1 step! Awesome!");
                } else {
                    println!("You win the game in {} steps! Cool!", step_counter);
                }

                break;
            }
        }
    }
}
