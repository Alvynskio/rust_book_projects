use std::cmp::Ordering;
use std::io;
use std::io::Write;
use rand::Rng;

pub fn run() {
    println!("Welcome to the guessing game!");

    let secret_num = rand::thread_rng().gen_range(0..101);
    loop {
        print!("Please enter your input: ");
        io::stdout().flush().unwrap(); // ensure above line prints immediately

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        };

    }
}