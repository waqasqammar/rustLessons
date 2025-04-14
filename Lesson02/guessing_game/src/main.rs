use std::io; // import io module
use rand::Rng;
use std::cmp::Ordering;
use colored::Colorize;

fn main() {
    println!("{}", "GUESS THE NUMBER".blue().bold());
    

    let secret_number = rand::rng().random_range(1..=100); // generate a random number
    println!("The secret number is: {}", secret_number);
    loop { 
        println!("Please input your guess.");
        let mut guess = String::new(); // create a mutable string to store user input

        io::stdin ()
            .read_line(&mut guess)
            .expect("Failed to read line"); // method to call on Result to handle error in case of failure


        println!( "You guess: {}", guess);   

        let guess :u32 = guess.trim().parse().expect("Please Input a number."); // variable shadowing

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small!!".red()),
            Ordering::Greater => println!("{}",  "Too Big!!".red()),
            Ordering::Equal => {
                println!("{}", " You Win!!".green());
                break;
            }
        }
    }
}

