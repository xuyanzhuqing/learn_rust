use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("game start");
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("the secret number is: {}", secret_number);

    println!("please input your guess");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("you guess: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a number");
                continue;
            },
        };
        println!("you guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
