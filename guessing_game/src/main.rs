use std::io;
use std::cmp::Ordering;


use rand::Rng;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Please enter a valid number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("You entered: {number}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    if number % 2 == 0 {
        println!("{} is even.", number);
    } else {
        println!("{} is odd.", number);
    }
}