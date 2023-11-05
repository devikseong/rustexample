extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("SecretNumber {} ", secret_number);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Error parse");
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Less"),
        Ordering::Greater => println!("Greater"),
        Ordering::Equal => println!("Equal"),
    }


    println!("you guessed {}", guess)
}
