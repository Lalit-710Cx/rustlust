use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    print!("Guess the number! ");

    let unique_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("The guess number is {unique_number}");

        println!("Lets Enter your secret Number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed the data type");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your guess is {guess}");

        match guess.cmp(&unique_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Won");
                break;
            }
        }
    }
}
