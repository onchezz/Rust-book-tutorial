use rand::Rng;
use std::cmp::Ordering;
use std::io;

const TRIALS: u32 = 5;
fn main() {
    println!("Guess a number ");
    let mut trials_remaining = TRIALS;
    let secret_number = rand::thread_rng().gen_range(0..100);
    println!("The secret number is  {}", secret_number);
    loop {
        println!("Please enter a  number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Greater => {
                trials_remaining -= 1;
                println!("Too high")
            }
            Ordering::Less => {
                trials_remaining -= 1;
                println!("Too low ")
            }
            Ordering::Equal => {
                println!("You win the Game!!");

                break;
            }
        }
        if trials_remaining == 0 {
            println!("Sorry you lost The Game !!");
            break;
        }
        println!(
            "Your guess is {} and trials remaing : {}",
            guess, trials_remaining
        );
    }
}
