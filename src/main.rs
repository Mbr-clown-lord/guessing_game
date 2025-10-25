use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");
    loop {
        println!("Please input your guess!");

        let mut guess = String::new();
        let ret = io::stdin().read_line(&mut guess);
        if let Err(e) =ret{
            eprintln!("Read io error: {:?}",e);
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed:{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
