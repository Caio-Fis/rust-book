use std::io;
use rand::Rng;
//use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    println!("Please input the maximun number");

    let mut max_number = String::new();

    io::stdin()
        .read_line(&mut max_number)
        .expect("Failed to read line");
    
    let max_number: u32 = max_number.trim().parse().expect("Please type a number");

    let near_number: f32 = (max_number/10) as f32;

    let secret_number = rand::thread_rng().gen_range(1..= max_number);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess > secret_number {
            if ((guess - secret_number) as f32) >= (near_number) {
                println!("Too big!")
            } else {
                println!("bit bigger!")
            }
        } else if guess < secret_number {
            if ((secret_number - guess) as f32) >= (near_number) {
                println!("Too small!")
            } else {
                println!("bit smaller!")
            }
        } else {
            println!("You win!"); 
            break;
        }
        /*
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!"); 
                break;}
        }
        */
    
    }

    
}
